import os
import random

import discord

TOKEN = os.environ.get("DISCORD_CHORROR_TOKEN")
client = discord.Client()

with open("common", "r") as f:
    common = [line for line in f.readlines()]
with open("uncommon", "r") as f:
    uncommon = [line for line in f.readlines()]
with open("rare", "r") as f:
    rare = [line for line in f.readlines()]
with open("extrarare", "r") as f:
    extrarare = [line for line in f.readlines()]


def format_msg(base_msg, rarity):
    if "||" in base_msg or "~~" in base_msg:
        # remove the newline character
        return base_msg[:-1]

    if rarity == 1:
        # dark orange
        # msg = f"```css\n[ {base_msg[:-1]} ]```"
        # red
        msg = f"```diff\n- {base_msg[:-1]} -```"
    elif rarity <= 10:
        # blue
        msg = f"```md\n# {base_msg[:-1]} #```"
    elif rarity <= 30:
        # turquoise
        msg = f"```cs\n' {base_msg[:-1]} '```"
    else:
        return base_msg

    return msg


@client.event
async def on_ready():
    print("logged in as {0.user}".format(client))


@client.event
async def on_message(message):
    if message.author == client.user:
        return

    if message.content.startswith("/hello"):
        await message.channel.send("ahoy hoy")
    elif message.content.startswith("/chore"):
        rarity = random.randint(1, 100)

        if rarity == 1:
            i = random.randint(0, len(extrarare) - 1)
            msg = format_msg(extrarare[i], rarity)

        elif rarity <= 10:
            i = random.randint(0, len(rare) - 1)
            msg = format_msg(rare[i], rarity)

        elif rarity <= 30:
            i = random.randint(0, len(uncommon) - 1)
            msg = format_msg(uncommon[i], rarity)

        else:
            i = random.randint(0, len(common) - 1)
            msg = f"```{common[i]}```"

        await message.channel.send(msg)



client.run(TOKEN)
