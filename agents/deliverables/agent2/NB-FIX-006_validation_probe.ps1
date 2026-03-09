$ErrorActionPreference = 'Stop'

$repo = 'C:\Users\jlaut\git\RetroGameGame'
$port = 15702
$gameExe = Join-Path $repo 'target\debug\retro-game-game.exe'

$startShot = Join-Path $repo 'agents\deliverables\agent2\NB-FIX-006_ground_start.png'
$moveShot = Join-Path $repo 'agents\deliverables\agent2\NB-FIX-006_ground_move.png'
$laneShot = Join-Path $repo 'agents\deliverables\agent2\NB-FIX-006_ground_lane.png'
$healthSetupShot = Join-Path $repo 'agents\deliverables\agent2\NB-FIX-006_health_setup.png'
$healthShot = Join-Path $repo 'agents\deliverables\agent2\NB-FIX-006_health_drop.png'
$validationJson = Join-Path $repo 'agents\deliverables\agent2\NB-FIX-006_brp_validation.json'

$cameraType = 'retro_game_game::eras::era_future::nebula_bouncer::components::NebulaGameplayCamera'
$playerType = 'retro_game_game::eras::era_future::nebula_bouncer::components::PlayerShip'
$hexType = 'retro_game_game::eras::era_future::nebula_bouncer::topography::TopographyHex'
$breakableType = 'retro_game_game::eras::era_future::nebula_bouncer::components::BreakableHazard'
$healthDropType = 'retro_game_game::eras::era_future::nebula_bouncer::components::HealthDrop'
$transformType = 'bevy_transform::components::transform::Transform'
$telemetryType = 'retro_game_game::eras::era_future::nebula_bouncer::resources::NebulaRuntimeTelemetry'
$procgenValidationType = 'retro_game_game::eras::era_future::nebula_bouncer::resources::NebulaProcgenValidationState'
$validationCommandType = 'retro_game_game::eras::era_future::nebula_bouncer::resources::NebulaValidationCommand'

function Invoke-Brp {
    param(
        [Parameter(Mandatory = $true)]
        [string]$Method,
        [Parameter()]
        $Params = $null,
        [Parameter()]
        [int]$Id = 1
    )

    $payload = [ordered]@{
        jsonrpc = '2.0'
        id = $Id
        method = $Method
    }
    if ($null -ne $Params) {
        $payload.params = $Params
    }

    Invoke-RestMethod `
        -Uri "http://127.0.0.1:$port" `
        -Method Post `
        -ContentType 'application/json' `
        -Body ($payload | ConvertTo-Json -Depth 16 -Compress)
}

function Wait-Until {
    param(
        [Parameter(Mandatory = $true)]
        [scriptblock]$Condition,
        [Parameter()]
        [int]$Attempts = 80,
        [Parameter()]
        [int]$DelayMs = 250,
        [Parameter()]
        [string]$FailureMessage = 'Condition not met before timeout.'
    )

    for ($i = 0; $i -lt $Attempts; $i++) {
        $result = & $Condition
        if ($null -ne $result) {
            return $result
        }
        Start-Sleep -Milliseconds $DelayMs
    }

    throw $FailureMessage
}

function Wait-File {
    param(
        [Parameter(Mandatory = $true)]
        [string]$Path
    )

    Wait-Until -Attempts 40 -DelayMs 250 -FailureMessage "Expected file was not created: $Path" -Condition {
        if (Test-Path $Path) {
            $item = Get-Item $Path
            if ($item.Length -gt 0) {
                return $item
            }
        }
        return $null
    } | Out-Null
}

function Query-SingleTransform {
    param(
        [Parameter(Mandatory = $true)]
        [string]$WithType,
        [Parameter()]
        [int]$Id = 100
    )

    $response = Invoke-Brp -Method 'world.query' -Id $Id -Params @{
        data = @{
            components = @($transformType)
            option = @()
            has = @()
        }
        filter = @{
            with = @($WithType)
            without = @()
        }
        strict = $false
    }

    if (-not $response.result -or $response.result.Count -eq 0) {
        throw "Query returned no entities for $WithType"
    }

    return $response.result[0]
}

function Query-EntitiesByType {
    param(
        [Parameter(Mandatory = $true)]
        [string]$WithType,
        [Parameter(Mandatory = $true)]
        [string[]]$Components,
        [Parameter()]
        [int]$Id = 400
    )

    $response = Invoke-Brp -Method 'world.query' -Id $Id -Params @{
        data = @{
            components = $Components
            option = @()
            has = @()
        }
        filter = @{
            with = @($WithType)
            without = @()
        }
        strict = $false
    }

    if ($response.result) {
        return @($response.result)
    }

    return @()
}

function Get-Translation {
    param(
        [Parameter(Mandatory = $true)]
        $EntityRecord
    )

    return $EntityRecord.components.$transformType.translation
}

function Select-CurrentHealthBreakable {
    param(
        [Parameter(Mandatory = $true)]
        [double[]]$ReferencePos,
        [Parameter()]
        [int]$Id = 450
    )

    $breakables = Query-EntitiesByType `
        -WithType $breakableType `
        -Components @($breakableType, $transformType) `
        -Id $Id

    return $breakables |
        Where-Object {
            $_.components.$breakableType.reward -eq 'HealthBearing'
        } |
        Sort-Object @{
            Expression = {
                $translation = $_.components.$transformType.translation
                [Math]::Abs($translation[1] - $ReferencePos[1]) +
                ([Math]::Abs($translation[0] - $ReferencePos[0]) * 0.40)
            }
        } |
        Select-Object -First 1
}

function Find-HealthDropNear {
    param(
        [Parameter(Mandatory = $true)]
        [double[]]$ReferencePos,
        [Parameter()]
        [int]$Id = 500
    )

    $drops = Query-EntitiesByType `
        -WithType $healthDropType `
        -Components @($healthDropType, $transformType) `
        -Id $Id

    if ($drops.Count -eq 0) {
        return $null
    }

    return $drops |
        Sort-Object @{
            Expression = {
                $translation = $_.components.$transformType.translation
                [Math]::Abs($translation[1] - $ReferencePos[1]) +
                ([Math]::Abs($translation[0] - $ReferencePos[0]) * 0.35)
            }
        } |
        Select-Object -First 1
}

if (-not (Test-Path $gameExe)) {
    throw "Game executable not found: $gameExe"
}

Get-Process retro-game-game -ErrorAction SilentlyContinue | Stop-Process -Force
Start-Sleep -Milliseconds 300

$env:RETRO_DEV_BOOT = 'nebula'
$env:BEVY_BRP_ENABLE = '1'
$env:BEVY_BRP_PORT = "$port"
$game = Start-Process -FilePath $gameExe -WorkingDirectory $repo -PassThru

try {
    $cameraRecord = Wait-Until -Attempts 100 -DelayMs 250 -FailureMessage 'BRP camera query did not succeed before timeout.' -Condition {
        try {
            Query-SingleTransform -WithType $cameraType -Id 101
        } catch {
            $null
        }
    }

    Invoke-Brp -Method 'brp_extras/screenshot' -Id 102 -Params @{ path = $startShot } | Out-Null
    Wait-File -Path $startShot

    $playerBefore = Query-SingleTransform -WithType $playerType -Id 103
    $cameraBefore = Query-SingleTransform -WithType $cameraType -Id 104
    $hexesBefore = Invoke-Brp -Method 'world.query' -Id 105 -Params @{
        data = @{
            components = @($transformType)
            option = @()
            has = @()
        }
        filter = @{
            with = @($hexType)
            without = @()
        }
        strict = $false
    }
    $playerBeforePos = Get-Translation -EntityRecord $playerBefore
    $trackedHexBefore = $hexesBefore.result |
        Where-Object {
            $_.components.$transformType.translation[1] -gt ($playerBeforePos[1] + 180.0)
        } |
        Sort-Object `
            @{ Expression = { [Math]::Abs($_.components.$transformType.translation[0]) } }, `
            @{ Expression = { $_.components.$transformType.translation[1] } } |
        Select-Object -First 1
    if (-not $trackedHexBefore) {
        throw 'Failed to find a center-lane TopographyHex to track.'
    }

    Invoke-Brp -Method 'brp_extras/send_keys' -Id 106 -Params @{
        keys = @('KeyD')
        duration_ms = 450
    } | Out-Null
    Start-Sleep -Milliseconds 900

    $playerAfter = Query-SingleTransform -WithType $playerType -Id 107
    $cameraAfter = Query-SingleTransform -WithType $cameraType -Id 108
    $trackedHexAfter = Invoke-Brp -Method 'world.get_components' -Id 109 -Params @{
        entity = $trackedHexBefore.entity
        components = @($transformType)
        strict = $false
    }

    Invoke-Brp -Method 'brp_extras/screenshot' -Id 110 -Params @{ path = $moveShot } | Out-Null
    Wait-File -Path $moveShot

    Invoke-Brp -Method 'brp_extras/screenshot' -Id 111 -Params @{ path = $laneShot } | Out-Null
    Wait-File -Path $laneShot

    $healthDestroyedTelemetry = $null
    $healthCollectionTelemetry = $null
    $healthBreakable = $null
    $healthBreakableTranslation = $null
    $shotSetupPlayer = $null
    for ($i = 0; $i -lt 12; $i++) {
        $currentPlayer = Query-SingleTransform -WithType $playerType -Id (120 + ($i * 8))
        $currentPlayerPos = Get-Translation -EntityRecord $currentPlayer
        $candidate = Select-CurrentHealthBreakable -ReferencePos $currentPlayerPos -Id (121 + ($i * 8))
        if (-not $candidate) {
            Start-Sleep -Milliseconds 200
            continue
        }

        $candidateTranslation = $candidate.components.$transformType.translation
        $playerSetupX = if ([Math]::Abs($candidateTranslation[0]) -lt 84.0) {
            [double]$candidateTranslation[0]
        } else {
            [double]($candidateTranslation[0] - ([Math]::Sign($candidateTranslation[0]) * 84.0))
        }
        $shotSetupPlayer = @(
            $playerSetupX,
            [double]($candidateTranslation[1] - 210.0),
            [double]$currentPlayerPos[2]
        )
        $fireOrigin = @(
            $playerSetupX,
            [double]($candidateTranslation[1] - 170.0),
            [double]$currentPlayerPos[2]
        )

        Invoke-Brp -Method 'world.insert_resources' -Id (200 + ($i * 3)) -Params @{
            resource = $validationCommandType
            value = @{
                teleport_player = $shotSetupPlayer
                fire_world_origin = $null
                fire_world_target = $null
            }
        } | Out-Null
        Start-Sleep -Milliseconds 250

        Invoke-Brp -Method 'brp_extras/screenshot' -Id (201 + ($i * 3)) -Params @{ path = $healthSetupShot } | Out-Null
        Wait-File -Path $healthSetupShot

        Invoke-Brp -Method 'world.insert_resources' -Id (202 + ($i * 3)) -Params @{
            resource = $validationCommandType
            value = @{
                teleport_player = $shotSetupPlayer
                fire_world_origin = $fireOrigin
                fire_world_target = $candidateTranslation
            }
        } | Out-Null
        Start-Sleep -Milliseconds 500

        $telemetryResponse = Invoke-Brp -Method 'world.get_resources' -Id (230 + $i) -Params @{
            resource = $telemetryType
        }
        $telemetry = $telemetryResponse.result.value
        if ($telemetry.health_breakables_destroyed -ge 1) {
            $healthDestroyedTelemetry = $telemetry
            $healthBreakable = $candidate
            $healthBreakableTranslation = $candidateTranslation
            break
        }
    }

    if (-not $healthDestroyedTelemetry) {
        throw 'Health-bearing breakable did not register as destroyed during live validation.'
    }

    $healthDrop = Wait-Until -Attempts 40 -DelayMs 250 -FailureMessage 'HealthDrop entity did not appear after health-bearing breakable destruction.' -Condition {
        Find-HealthDropNear -ReferencePos $healthBreakableTranslation -Id 300
    }

    Start-Sleep -Milliseconds 350
    Invoke-Brp -Method 'brp_extras/screenshot' -Id 301 -Params @{ path = $healthShot } | Out-Null
    Wait-File -Path $healthShot

    $collectPlayer = @(
        [double]$healthDrop.components.$transformType.translation[0],
        [double]$healthDrop.components.$transformType.translation[1],
        [double]$shotSetupPlayer[2]
    )
    Invoke-Brp -Method 'world.insert_resources' -Id 302 -Params @{
        resource = $validationCommandType
        value = @{
            teleport_player = $collectPlayer
            fire_world_origin = $null
            fire_world_target = $null
        }
    } | Out-Null

    $healthCollectionTelemetry = Wait-Until -Attempts 40 -DelayMs 250 -FailureMessage 'HealthDrop was visible but did not confirm collection in runtime telemetry.' -Condition {
        $telemetryResponse = Invoke-Brp -Method 'world.get_resources' -Id 303 -Params @{
            resource = $telemetryType
        }
        $telemetry = $telemetryResponse.result.value
        $dropStillPresent = Find-HealthDropNear -ReferencePos $healthBreakableTranslation -Id 304

        if ($telemetry.player_health_recovered -ge $healthDrop.components.$healthDropType.heal_amount -and -not $dropStillPresent) {
            return $telemetry
        }

        return $null
    }

    $procgenValidation = Invoke-Brp -Method 'world.get_resources' -Id 305 -Params @{
        resource = $procgenValidationType
    }
    $startupSequence = $procgenValidation.result.value.recent

    $laneHexSampleCount = ($hexesBefore.result | Where-Object {
        $translation = $_.components.$transformType.translation
        [Math]::Abs($translation[0]) -lt 140.0 -and $translation[1] -gt ($playerBeforePos[1] - 80.0) -and $translation[1] -lt ($playerBeforePos[1] + 1400.0)
    }).Count

    $validation = [ordered]@{
        movement_case = [ordered]@{
            player_before = $playerBeforePos
            player_after = (Get-Translation -EntityRecord $playerAfter)
            camera_before = (Get-Translation -EntityRecord $cameraBefore)
            camera_after = (Get-Translation -EntityRecord $cameraAfter)
            tracked_hex_entity = $trackedHexBefore.entity
            tracked_hex_before = $trackedHexBefore.components.$transformType.translation
            tracked_hex_after = $trackedHexAfter.result.components.$transformType.translation
            start_screenshot = $startShot
            move_screenshot = $moveShot
        }
        coherent_floor_case = [ordered]@{
            startup_sequence = $startupSequence
            center_lane_hex_sample_count = $laneHexSampleCount
            lane_screenshot = $laneShot
        }
        health_drop_case = [ordered]@{
            breakable_entity = $healthBreakable.entity
            breakable_translation = $healthBreakableTranslation
            breakable_reward = $healthBreakable.components.$breakableType.reward
            shot_setup_player = $shotSetupPlayer
            pre_shot_screenshot = $healthSetupShot
            runtime_telemetry_after_destroy = $healthDestroyedTelemetry
            health_drop_entity = $healthDrop.entity
            health_drop_translation = $healthDrop.components.$transformType.translation
            health_drop_component_at_capture = $healthDrop.components.$healthDropType
            drop_screenshot = $healthShot
            runtime_telemetry_after_collection = $healthCollectionTelemetry
        }
        artifact_paths = [ordered]@{
            validation_json = $validationJson
            start_screenshot = $startShot
            move_screenshot = $moveShot
            lane_screenshot = $laneShot
            health_setup_screenshot = $healthSetupShot
            health_drop_screenshot = $healthShot
        }
    }

    $validation | ConvertTo-Json -Depth 12 | Set-Content -Path $validationJson
    $validation | ConvertTo-Json -Depth 12
}
finally {
    if ($game -and -not $game.HasExited) {
        Stop-Process -Id $game.Id -Force
    }
}
