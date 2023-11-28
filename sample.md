curl --request POST \
  --url https://api.openai.com/v1/audio/transcriptions \
  -H "Authorization: Bearer $OPENAI_API_KEY" \
  -H "Content-Type: multipart/form-data" \
  --form file=@/fixtures/alloy.mp3 \
  --form model=whisper-1