
from aiogram.client.telegram import TelegramAPIServer


class TginUpateLongPullServer(TelegramAPIServer):
    tgin_updates: str

    def __init__(self, tgin_updates: str, **kwargs):
        self.tgin_updates = tgin_updates
        if "base" not in kwargs:
            kwargs["base"] = "https://api.telegram.org/bot{token}/{method}"
        if "file" not in kwargs:
            kwargs["file"] = "https://api.telegram.org/file/bot{token}/{path}"
        super().__init__(**kwargs)

    def api_url(self, token: str, method: str) -> str:
        if method == "getUpdates":
            return self.tgin_updates
        return self.base.format(token=token, method=method)

