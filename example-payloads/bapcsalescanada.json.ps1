$url = "https://www.reddit.com/r/bapcsalescanada.json"
$resp = Invoke-WebRequest -Uri $url
$resp.Content | ConvertFrom-Json -Depth 100 | ConvertTo-Json -Depth 100 | Out-File -FilePath "bapcsalescanada.json"