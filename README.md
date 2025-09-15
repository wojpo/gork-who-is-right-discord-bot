# Gork who is right discord bot

## Overview

This is a Discord bot built in Rust using the Poise framework, designed to analyze recent messages in a channel and determine who is right in a conversation. It fetches up to 100 messages, sends the context to the Gemini API, and returns the API’s verdict in chunks to comply with Discord’s 2000-character message limit.

## Features

- Slash Command: Use `/who_is_right` to trigger the bot.

- Customizable Message Count: Specify how many messages to analyze (default: 50, max: 100).

- Gemini API Integration: Sends conversation context to the Gemini API to determine who is right.

- Chunked Responses: Splits long responses into multiple messages to fit Discord’s limits.
