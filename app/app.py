import json
import logging
import os
import requests
from telegram import Message, Update
from telegram.ext import ApplicationBuilder, ContextTypes, CommandHandler, MessageHandler, filters
from dotenv import load_dotenv

load_dotenv()
TELEGRAM_BOT_TOKEN = os.getenv('TELEGRAM_BOT_TOKEN')

logging.basicConfig(
    format='%(asctime)s - %(name)s - %(levelname)s - %(message)s',
    level=logging.INFO
)

NO_REQUEST, ADDRESS_REQUEST, HAS_ADDRESS = range(3)
has_address = {}
chat_message_addresses = {}
chat_message_proofs = {}
chat_message_ids = {}
chat_message_girlfriend = {}

class ProofMessage:
    def __init__(self, message: Message):
        self.fwd_origin = message.forward_origin
        self.text = message.text

    def to_dict(self):
        return {
            'forward_origin': self.fwd_origin.to_dict(),
            'text': self.text
        }
    
    def to_json(self):
        return json.dumps(self.to_dict())

async def help(update: Update, context: ContextTypes.DEFAULT_TYPE):

    prompt = """
    Prove you have a Girlfriend by forwarding a minimum of 3 messages from her.
    \nUse /start to begin the process.
    \nUse /verify to start the Zero-Knowledge Proof process.
    \nThe more messages you forward, with evenly spread out dates the better!
    \nAccepted Regex Matches:
    \n\t- "i love you"
    \n\t- "❤️"
    """

    chat_id = update.effective_chat.id
    await context.bot.send_message(chat_id=chat_id, text=prompt)


async def start(update: Update, context: ContextTypes.DEFAULT_TYPE):

    prompt = """
    Prove you have a Girlfriend by forwarding a minimum of 3 messages from her.
    \nThe more messages you forward, with evenly spread out dates the better!
    \nAccepted Regex Matches:
    \n\t- "i love you"
    \n\t- "❤️"
    """
    question = "Please submit your ethereum address to receive a NFT of your proof."

    chat_id = update.effective_chat.id
    chat_message_proofs[chat_id] = []
    chat_message_ids[chat_id] = []
    has_address[chat_id] = ADDRESS_REQUEST

    await context.bot.send_message(chat_id=chat_id, text=prompt)
    await context.bot.send_message(chat_id=chat_id, text=question)


async def verify(update: Update, context: ContextTypes.DEFAULT_TYPE):
    
    chat_id = update.effective_chat.id
    proofs = chat_message_proofs.get(chat_id, [])

    if has_address.get(chat_id, NO_REQUEST) != HAS_ADDRESS:
        await context.bot.send_message(chat_id=chat_id, text="Please submit your ethereum address to continue.")
        return 
    if len(proofs) < 3:
        await context.bot.send_message(chat_id=chat_id, text="Please forward at least 3 messages to verify.")
        return
    
    jsonproofs = [proof.to_json() for proof in proofs]

    # call the verifier endpoint localhost:3000
    # POST /verify

    headers = {
        'Content-Type': 'application/json',
    }
    body ={
        'address': chat_message_addresses[chat_id],
        'proofs': jsonproofs
    }
    print(body)
    r = requests.post('http://localhost:3000', headers=headers, data=body)
    print(r.status_code)
    print(r)

    # generating proofs...
    await context.bot.send_message(chat_id=chat_id, text="Generating Zero-Knowledge Proofs...")

    if r.status_code == 200:
        await context.bot.send_message(chat_id=chat_id, text="Verification Successful! You will receive your NFT shortly.")
    else:
        await context.bot.send_message(chat_id=chat_id, text="Verification Failed! Please try again.")

async def respond(update: Update, context: ContextTypes.DEFAULT_TYPE):
    
    chat_id = update.effective_chat.id
    girlfriend = update.message.forward_origin.sender_user.username
    message_id = hash(str(update.message.forward_origin.date) + update.message.text)

    if has_address.get(chat_id, NO_REQUEST) != HAS_ADDRESS:
        await context.bot.send_message(chat_id=chat_id, text="Please submit your ethereum address to continue.")
        return

    if chat_id in chat_message_proofs:

        # check if the sender girlfriend has been set
        if chat_id not in chat_message_girlfriend:
            chat_message_girlfriend[chat_id] = girlfriend

        # check if the sender is the girlfriend
        if chat_message_girlfriend[chat_id] != girlfriend:
            await context.bot.send_message(
                chat_id=chat_id, 
                text="Please forward messages from your girlfriend: {}".format(chat_message_girlfriend[chat_id])
            )   
            return
        
        # check if the message has been already forwarded before
        if message_id in chat_message_ids[chat_id]:
            await context.bot.send_message(
                chat_id=chat_id, 
                text="You have already forwarded this message."
            )
            return

        proof_message = ProofMessage(update.message)
        chat_message_proofs[chat_id].append(proof_message)
        chat_message_ids[chat_id].append(message_id)
        
        await context.bot.send_message(
            chat_id=chat_id, 
            text="Thank you for the forwarded message. You have forwarded {} messages so far.".format(len(chat_message_proofs[chat_id]))
        )
    
    else: await context.bot.send_message(chat_id=chat_id, text="Please use /start to begin the process.")


async def regular(update: Update, context: ContextTypes.DEFAULT_TYPE):
    chat_id = update.effective_chat.id

    if has_address.get(chat_id, NO_REQUEST) == ADDRESS_REQUEST:

        address = update.message.text

        if not address.startswith('0x') or len(address) != 42:
            await context.bot.send_message(
                chat_id=chat_id, 
                text="Please submit a valid ethereum address."
            )
            return

        chat_message_addresses[chat_id] = address
        has_address[chat_id] = HAS_ADDRESS
        await context.bot.send_message(
            chat_id=chat_id, 
            text="Thank you for submitting your ethereum address. Please forward messages from your girlfriend."
        )
        return

    await context.bot.send_message(
        chat_id=chat_id, 
        text="Please forward messages only, or use /start to (re)start the process. You are currently at {} forwarded messages.".format(len(chat_message_proofs.get(chat_id, []))))

if __name__ == '__main__':
    application = ApplicationBuilder().token(TELEGRAM_BOT_TOKEN).build()

    help_handler = CommandHandler('help', help)
    application.add_handler(help_handler)
    
    start_handler = CommandHandler('start', start)
    application.add_handler(start_handler)

    verify_handler = CommandHandler('verify', verify)
    application.add_handler(verify_handler)

    respond_handler = MessageHandler(filters.TEXT & filters.FORWARDED, respond)
    application.add_handler(respond_handler)

    message_handler = MessageHandler(filters.TEXT, regular)
    application.add_handler(message_handler)

    application.run_polling()
