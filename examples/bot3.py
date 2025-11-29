import logging
import sys
from aiohttp import web
from aiogram import Bot, Dispatcher, types
from aiogram.enums import ParseMode
from aiogram.filters import CommandStart
from aiogram.webhook.aiohttp_server import SimpleRequestHandler, setup_application


dp = Dispatcher()
bot = Bot(token="8546245682:AAEepqOlAZmm2joO7B4z90T72a0utrNjhJY")



@dp.message()
async def echo_handler(message: types.Message):
    await message.answer("это третий бот с вебхуком")

def main():
    logging.basicConfig(level=logging.INFO, stream=sys.stdout)

    app = web.Application()


    webhook_requests_handler = SimpleRequestHandler(
        dispatcher=dp,
        bot=bot,
    )

    webhook_requests_handler.register(app, path="/bot")

    setup_application(app, dp, bot=bot)


    web.run_app(app, host="127.0.0.1", port=3001)

if __name__ == "__main__":
    main()