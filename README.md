# Gork who is right discord bot
## Backstory

During our Hack4Krak project, [@wikipop](github.com/wikipop) and [@norbiros](github.com/norbiros) frequently clashed in our Discord channel with heated debates. Tired of their arguments, I created the "Gork Who Is Right" bot, inspired by the popular "Grok" meme, to settle disputes by analyzing conversations with the Gemini API and declaring a winner.

## Overview

This is a Discord bot built in Rust using the Poise framework, designed to analyze recent messages in a channel and determine who is right in a conversation. It fetches up to 100 messages, sends the context to the Gemini API, and returns the API’s verdict in chunks to comply with Discord’s 2000-character message limit.

## Features

- Slash Command: Use `/who_is_right` to trigger the bot.

- Customizable Message Count: Specify how many messages to analyze (default: 50, max: 100).

- Gemini API Integration: Sends conversation context to the Gemini API to determine who is right.

- Chunked Responses: Splits long responses into multiple messages to fit Discord’s limits.
