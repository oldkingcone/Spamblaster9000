# Rusty Spammer!
This tool does what it says in the name, it spams.
This tool is best used against scammers, phishers, and other malicious actors.
Why? Because it's fun.
Why not fill up the database with spam, alert logs, or other malicious tracking these actors are using.
Waste their time if you find their infrastructure.

# What this targets
This tool targets the following:
- Discord
- Telegram
- WhatsApp (defined in the generic type)
- Slack
- Discord Webhook
- Telegram Webhook (defined in the generic type.)
- WhatsApp Webhook
- Slack Webhook

# Example json file:
```json
{
  "target_url": "https://httpbin.org/anything",
  "target_method": "POST",
  "target_body": {
    "foo": "bar"
  },
  "target_headers": {
    "Content-Type": "application/json"
  }
}
```
I havent yet setup the target_headers yet, but that can and will be added someday. This is just a side project for me.

Have fun, and remember to check out my other projects.


