# Pigeon

Pigeon is a command line tool for automating your email workflow in a cheap and efficient way. Utilize your most efficient dev tools you are already familiar with.

For example, you can define a bash `alias` with your individual `pigeon` command to send your weekly newsletter to your audience. You might want to automatize the send schedule by defining a systemd `.service`.  

You might also draft a static html with your favorite web development framework, and use pigeon to send this html template.

- [Requirements](#requirements)
- [Install Pigeon](#install-pigeon)
  - [Install Pigeon from crates.io](#install-pigeon-from-crates-io)
  - [Install Pigeon from github.com](#install-pigeon-form-github-com)
- [Usage](#usage)
  - [Send email to a single receiver](#send-email-to-a-single-receiver)
  - [Send bulk email to multiple receivers](#send-bulk-email-to-multiple-receivers)
  - [Personalize your emails](#personalize-your-emails)
- [How to connect](#how-to-connect)
  - [How to connect to email provider](#how-to-connect-to-email-provider)
  - [How to connect to postgres database](#how-to-connect-to-postgres-database)
- [Integrations](#integrations)
  - [Email provider](#email-provider)
  - [Data sources](#data-sources)
- [Comparison with Mailchimp and Sendgrid](#comparison-with-mailchimp-and-sendgrid)

## Requirements

You need to have Rust installed on your system and nightly toolchain activated.

## Install Pigeon

### Install Pigeon from crates.io

``` bash
# Install nightly toolchain
rustup toolchain install nightly

# Switch to nightly toolchain
rustup override set nightly

# Build and install pigeon binary to ~/.cargo/bin
cargo install pigeon-rs
```

### Install Pigeon from github.com

``` bash
# Clone repository
git clone git@github.com:quambene/pigeon-rs.git
cd pigeon-rs

# Activate rust nightly toolchain for current directory
echo "nightly" > rust-toolchain

# Build and install pigeon binary to ~/.cargo/bin
cargo install --path .
```

_Note:_ Add `$HOME/.cargo/bin` to your `PATH` if it is missing:

``` bash
export PATH="$HOME/.cargo/bin:$PATH"
```

## Usage

Check connection to your email provider with `pigeon connect`. For example, using AWS Simple Email Service (SES):

``` bash
pigeon connect aws
```

> Connected to aws client: <span style="color:MediumSeaGreen">ok</span>

See currently supported [integrations](#integrations) and [how to connect](#how-to-connect) below.

### Send email to a single receiver

Send a single email with subject and content:

``` bash
pigeon send sender@your-domain.com receiver@gmail.com --subject "Test subject" --content "This is a test email."
```

Send a single email with message defined in separate template file:

``` bash
pigeon send sender@your-domain.com receiver@gmail.com --message-file "message.yaml"
```

The message template `message.yaml` is created with subcommand `init`:

``` bash
pigeon init
```

_Note:_ One of the advantages of a `--message-file` is that you can also draft the html version of your email. In contrast, with the options `--subject` and `--object` the same format will be sent as plaintext and html email.

### Send bulk email to multiple receivers

For example, query relevant users which confirmed to receive your newsletter, and send an email to all of them. 

Let's check the query first via `pigeon query`:

``` bash
pigeon query --display "select email from user where newsletter_confirmed = true"
```

``` console
> Display query result: shape: (4, 1)
+------------------------------+
| email                        |
| ---                          |
| str                          |
+==============================+
| "marie@curie.com"            |
+------------------------------+
| "alexandre@grothendieck.com" |
+------------------------------+
| "emmy@noether.com"           |
+------------------------------+
| "elie@cartan.com"            |
+------------------------------+
```

See [how to connect](#how-to-connect) below to connect your database.

_Note:_ You can also `--save` your query as a csv file: `pigeon query --save <my-query>`.

Now send your newsletter to the queried receivers. Let's try a `--dry-run` without confirmation `--assume-yes` first:

``` bash
pigeon send-bulk albert@einstein.com --receiver-query "select email from user where newsletter_confirmed = true" --message-file "message.yaml" --assume-yes --dry-run
```

``` console
> Sending email to 4 receivers ...
marie@curie.com ... dry run
alexandre@grothendieck.com ... dry run
emmy@noether.com ... dry run
elie@cartan.com ... dry run
```

After double checking, you can submit the same command without `--dry-run`. Remove `--assume-yes` as well for explicit confirmation.

_Note:_ You can also send a bulk email to email adresses defined in a csv file instead of a query result. In this case, use option `--receiver-file` instead of `--receiver-query`. You can check the contents of a csv file via subcommand `read`, e.g. `pigeon read recipients.csv`.

### Personalize your emails

If you need more individual emails, you can _personalize_ your emails with option `--personalize`. Again, let's start by checking the relevant query:

``` bash
pigeon query --display "select first_name, last_name, email from user where newsletter_confirmed = true"
```

``` console
> Display query result: shape: (4, 3)
+-------------+----------------+------------------------------+
| first_name  | last_name      | email                        |
| ---         | ---            | ---                          |
| str         | str            | str                          |
+=============+================+==============================+
| "Marie"     | "Curie"        | "marie@curie.com"            |
+-------------+----------------+------------------------------+
| "Alexandre" | "Grothendieck" | "alexandre@grothendieck.com" |
+-------------+----------------+------------------------------+
| "Emmy"      | "Noether"      | "emmy@noether.com"           |
+-------------+----------------+------------------------------+
| "Elie"      | "Cartan"       | "elie@cartan.com"            |
+-------------+----------------+------------------------------+
```

In your message template `message.yaml` use variables in curly brackets, like `{first_name}` and `{last_name}`. Then define personalized colums as parameters for option `--personalize`. Finally, let's display everything with `--display`:

``` bash
pigeon send-bulk albert@einstein.com --receiver-query "select first_name, last_name, email from user where newsletter_confirmed = true" --message-file "message.yaml" --personalize "first_name" "last_name" --display
```

``` console
> Display message file: MessageTemplate {
    message: Message {
        subject: "Issue No. 1",
        text: "Dear {first_name} {last_name},
            Welcome to my newsletter. We are doing hard sciences here.
            Sincerely, Albert Einstein",
        html: "Dear {first_name} {last_name},
            Welcome to my newsletter. We are doing hard sciences here.
            Sincerely, Albert Einstein",
    },
}
> Display emails: BulkEmail {
    emails: [
        Email {
            sender: "albert@einstein.com",
            receiver: "marie@curie.com",
            message: Message {
                subject: "Issue No. 1",
                text: "Dear Marie Curie,
                    Welcome to my newsletter. We are doing hard sciences here.
                    Sincerely, Albert Einstein",
                html: "Dear Marie Curie,
                    Welcome to my newsletter. We are doing hard sciences here.
                    Sincerely, Albert Einstein",
                },
        },
        Email {
            sender: "albert@einstein.com",
            receiver: "alexandre@grothendieck.com",
            message: Message {
                subject: "Issue No. 1",
                text: "Dear Alexandre Grothendieck,
                    Welcome to my newsletter. We are doing hard sciences here.
                    Sincerely, Albert Einstein",
                html: "Dear Alexandre Grothendieck,
                    Welcome to my newsletter. We are doing hard sciences here.
                    Sincerely, Albert Einstein",
            },
        },
        Email {
            sender: "albert@einstein.com",
            receiver: "emmy@noether.com",
            message: Message {
                subject: "Issue No. 1",
                text: "Dear Emmy Noether,
                    Welcome to my newsletter. We are doing hard sciences here.
                    Sincerely, Albert Einstein",
                html: "Dear Emmy Noether,
                    Welcome to my newsletter. We are doing hard sciences here.
                    Sincerely, Albert Einstein",
            },
        },
        Email {
            sender: "albert@einstein.com",
            receiver: "elie@cartan.com",
            message: Message {
                subject: "Issue No. 1",
                text: "Dear Elie Cartan,
                    Welcome to my newsletter. We are doing hard sciences here.
                    Sincerely, Albert Einstein",
                html: "Dear Elie Cartan,
                    Welcome to my newsletter. We are doing hard sciences here.
                    Sincerely, Albert Einstein",
            },
        },
    ],
}
> Should an email be sent to 4 recipients? Yes (y) or no (n)
>
```

Confirm `y` if you are ready to go.

## How to connect

### How to connect to email provider

For AWS SES, define environment variables `AWS_ACCESS_KEY_ID` and `AWS_SECRET_ACCESS_KEY`. Source your environment `.env` in your current shell:

``` bash
set -a && source .env && set +a
```

### How to connect to postgres database

For postgres, the database url is constructed as follows: `postgresql://db_user:db_password@db_host:db_port/db_name`.

Therefore, set the following environment variables in your environment `.env`:

- `DB_HOST`
- `DB_PORT`
- `DB_USER`
- `DB_PASSWORD`
- `DB_NAME`

Source your environment again:

``` bash
set -a && source .env && set +a
```

## Integrations

### Email provider

- AWS SES

### Data sources

- PostgreSQL
- CSV

## Comparison with Mailchimp and Sendgrid

These numbers may be outdated. Do your own research.

emails/month | Pigeon+AWS* | Mailchimp Marketing* | Sendgrid API* | Sendgrid Marketing*
--------- | --------- | --------- | --------- | ---------
5,000 | $4 | $9.99 | $14.95 | $15
10,000 | $4 | $20.99 | $14.95 | $15
100,000 | $4 | $78.99 | $29.95 | $120

*Price per month

provider | daily limit
--------- | ---------
Pigeon+AWS | 50,000
Mailchimp | equals monthly limit
Sendgrid | equals monthly limit
