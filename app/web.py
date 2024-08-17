import os
import requests
from flask import Flask, request, jsonify
from dotenv import load_dotenv

load_dotenv()
TELEGRAM_BOT_TOKEN = os.getenv('TELEGRAM_BOT_TOKEN')

TELEGRAM_API_URL = f'https://api.telegram.org/bot{TELEGRAM_BOT_TOKEN}/'

app = Flask(__name__)

@app.route('/webhook', methods=['POST'])
def webhook():
    # Log raw request details
    print(request)
    print(request.remote_addr)
    print(request.authorization)
    raw_request = {
        'headers': dict(request.headers),
        'json': request.json,
        'data': request.get_data(as_text=True)
    }
    print("Raw Request Data:", raw_request)

    # Process the update
    update = request.json
    chat_id = update['message']['chat']['id']
    text = update['message']['text']

    # Echo the message back
    response = requests.post(
        f'{TELEGRAM_API_URL}sendMessage',
        json={'chat_id': chat_id, 'text': text}
    )

    # Log raw response details
    print("Raw Response Data:", response.json())

    return jsonify({'status': 'ok'})

if __name__ == '__main__':
    app.run(port=5000)
