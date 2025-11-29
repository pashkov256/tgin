import asyncio
import logging
import sys
from os import getenv

from dataclasses import dataclass

from aiogram import Bot, Dispatcher
from aiogram.client.default import DefaultBotProperties
from aiogram.enums import ParseMode
from aiogram.filters import CommandStart
from aiogram.types import Message

from aiogram.client.session.aiohttp import AiohttpSession
from aiogram.client.telegram import TelegramAPIServer

TOKEN = "..."

from pytgin.aiogram import TginUpateServer


server = TginUpateServer(
    tgin_updates= f"http://127.0.0.1:3000/bot2/getUpdates",
)


session = AiohttpSession(api=server)

dp = Dispatcher()


@dp.message()
async def echo_handler(message: Message) -> None:
    await message.answer("привет со второго бота")


async def main() -> None:
    bot = Bot(token=TOKEN, default=DefaultBotProperties(parse_mode=ParseMode.HTML), session=session)

    await dp.start_polling(bot)


if __name__ == "__main__":
    logging.basicConfig(level=logging.INFO, stream=sys.stdout)
    asyncio.run(main())