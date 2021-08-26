import os
import json
import random

import discord

TOKEN = os.environ.get("DISCORD_CHORROR_TOKEN")
CHOREBOT_CHANNEL_NAME = os.environ.get("CHOREBOT_CHANNEL_NAME")
client = discord.Client()


class Rarity:
    EXTRA = 1
    RARE = 10
    UNCOMMON = 30
    COMMON = 100

    def from_str(s):
        if s == "extra":
            return Rarity.EXTRA
        elif s == "rare":
            return Rarity.RARE
        elif s == "uncommon":
            return Rarity.UNCOMMON
        elif s == "common":
            return Rarity.COMMON
        else:
            return None

    def to_string(n):
        if n <= Rarity.EXTRA:
            return "extra"
        elif n <= Rarity.RARE:
            return "rare"
        elif n <= Rarity.UNCOMMON:
            return "uncommon"
        else:
            return "common"


# get count
chore_pool_count = 0
with open("common", "r") as f:
    common = f.readlines()
with open("uncommon", "r") as f:
    uncommon = f.readlines()
with open("rare", "r") as f:
    rare = f.readlines()
with open("extra", "r") as f:
    extra = f.readlines()
chore_pool_count += len(common) + len(uncommon) + len(rare) + len(extra)


data = {}
chorebot_channel_id = 0


async def private_msg(message):
    if message.content == "thanks":
        await message.author.send("no problem")


def has_decoration(msg):
    return "||" in msg or "~~" in msg


def format_msg(base_msg, rarity):
    if has_decoration(base_msg):
        # REMOVE THE NEWLINE CHARACTER
        return base_msg[:-1]

    if rarity <= Rarity.EXTRA:
        # dark orange
        # msg = f"```css\n[ {base_msg[:-1]} ]```"

        # red
        msg = f"```diff\n- {base_msg[:-1]} -```"

    elif rarity <= Rarity.RARE:
        # blue
        msg = f"```md\n# {base_msg[:-1]} #```"

    elif rarity <= Rarity.UNCOMMON:
        # turquoise
        msg = f"```cs\n' {base_msg[:-1]} '```"

    else:
        msg = f"```{base_msg[:-1]}```"

    return msg


@client.event
async def on_ready():
    global chorebot_channel, chorebot_channel_id, data
    print("logged in as {0.user}".format(client))

    if os.path.isfile("data.json"):
        print("data.json exists")
        with open("data.json", "r") as f:
            data = json.load(f)
    else:
        data = {
            "chorebot_channel": discord.utils.get(
                client.get_all_channels(), name=CHOREBOT_CHANNEL_NAME
            ).id
        }
        with open("data.json", "w") as f:
            f.write(json.dumps(data))

    if data.get("chorebot_channel") is False:
        raise Exception("Allowed channel unknown.")

    chorebot_channel_id = data["chorebot_channel"]
    chorebot_channel = client.get_channel(chorebot_channel_id)

    await chorebot_channel.send("**Choretle has grown stronger...**")


@client.event
async def on_message(message):
    global chorebot_channel, chore_pool_count, data

    if message.author == client.user:
        return

    allowed = message.channel.type == discord.ChannelType.private or (
        message.channel.type == discord.ChannelType.text
        and str(message.channel.id) == str(chorebot_channel_id)
    )

    if message.channel.type == discord.ChannelType.private:
        await private_msg(message)

    if message.content.lower().startswith("good morning") and allowed:
        await message.channel.send("**shrieks and beeps**")

    elif message.content.startswith("/chore"):
        if not allowed:
            await message.author.send(
                f"beep bloop! You can only ask for chores in {chorebot_channel}"
            )
            return

        rarity = random.randint(1, 100)
        rarity_s = Rarity.to_string(rarity)

        if rarity <= Rarity.EXTRA:
            with open("extra", "r") as f:
                extra = [line for line in f.readlines()]
            i = random.randint(0, len(extra) - 1)
            msg = format_msg(extra[i], rarity)

        elif rarity <= Rarity.RARE:
            with open("rare", "r") as f:
                rare = [line for line in f.readlines()]
            i = random.randint(0, len(rare) - 1)
            msg = format_msg(rare[i], rarity)

        elif rarity <= Rarity.UNCOMMON:
            with open("uncommon", "r") as f:
                uncommon = [line for line in f.readlines()]
            i = random.randint(0, len(uncommon) - 1)
            msg = format_msg(uncommon[i], rarity)

        else:
            with open("common", "r") as f:
                common = [line for line in f.readlines()]
            i = random.randint(0, len(common) - 1)
            msg = format_msg(common[i], rarity)

        await message.channel.send(msg)

        fname = str(message.author).split("#")[0] + ".json"
        try:
            with open(f"users/{fname}", "r") as f:
                user_data = json.load(f)
                if user_data.get(rarity_s) is None:
                    user_data[rarity_s] = 0
                user_data[rarity_s] += 1
            # data
        except Exception as e:
            print("error while reading json", e)
            user_data = {rarity_s: 1}

        try:
            with open(f"users/{fname}", "w+") as f:
                f.write(json.dumps(user_data, indent=2))
        except Exception as e:
            print("error while writing json", e)

    elif message.content.startswith("/add_c") and allowed:
        await add_chore(message, "common")

    elif message.content.startswith("/add_u") and allowed:
        await add_chore(message, "uncommon")

    elif message.content.startswith("/add_r") and allowed:
        await add_chore(message, "rare")

    elif message.content.startswith("/add_?") and allowed:
        await add_chore(message, "extra")

    if "choretle" in message.content.lower() and allowed:
        await message.add_reaction("👀")


async def add_chore(message, rarity_s):
    global chore_pool_count
    words = message.content.split()[1:]
    if len(words) == 0:
        await message.author.send("...?")
        await message.add_reaction("❌")
        return
    msg = " ".join(message.content.split()[1:]) + "\n"
    display_msg = format_msg(f"{msg}", Rarity.from_str(rarity_s))
    await message.author.send(f"adding \n{display_msg}")
    with open(rarity_s, "a") as f:
        f.write(msg)
    await message.add_reaction("👍")
    chore_pool_count += 1


client.run(TOKEN)
