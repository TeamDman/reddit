$url = "https://www.reddit.com/r/bapcsalescanada/comments/1iambwd/amd_ryzen_7_7700_23237_aliexpress.json"
$resp = Invoke-WebRequest -Uri $url
$resp.Content | ConvertFrom-Json -Depth 100 | ConvertTo-Json -Depth 100 | Out-File -FilePath "bapcsalescanada.post.json"