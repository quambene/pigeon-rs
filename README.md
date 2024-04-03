<!-- markdownlint-disable MD033 -->

# Pigeon

[![latest version](https://img.shields.io/crates/v/pigeon-rs.svg)](https://crates.io/crates/pigeon-rs)
[![documentation](https://docs.rs/pigeon-rs/badge.svg)](https://docs.rs/pigeon-rs/)
[![build status](https://github.com/quambene/pigeon-rs/actions/workflows/rust-ci.yml/badge.svg)](https://github.com/quambene/pigeon-rs/actions/workflows/rust-ci.yml)
[![codecov](https://codecov.io/gh/quambene/pigeon-rs/graph/badge.svg)](https://app.codecov.io/gh/quambene/pigeon-rs)
[![dependency status](https://deps.rs/repo/github/quambene/pigeon-rs/status.svg)](https://deps.rs/repo/github/quambene/pigeon-rs)

Pigeon is a command line tool for automating your email workflow in a cheap and efficient way. Utilize your most efficient dev tools you are already familiar with.

For example, query the subscribers of your newsletter, create a plaintext and html email from a template file, and send it to all of them:

``` bash
pigeon send-bulk \
    sender@your-domain.com \
    --receiver-query "select email from user where newsletter_confirmed = true" \
    --message-file "message.yaml" \
    --display \
    --assume-yes
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
> Sending email to 4 receivers ...
marie@curie.com ... ok
alexandre@grothendieck.com ... ok
emmy@noether.com ... ok
elie@cartan.com ... ok
```

- [Install Pigeon](#install-pigeon)
  - [Install Pigeon from crates.io](#install-pigeon-from-cratesio)
  - [Install Pigeon from github.com](#install-pigeon-from-githubcom)
- [Getting help](#getting-help)
- [Usage](#usage)
  - [Send email to a single receiver](#send-email-to-a-single-receiver)
  - [Send bulk email to multiple receivers](#send-bulk-email-to-multiple-receivers)
  - [Personalize your emails](#personalize-your-emails)
- [How to connect](#how-to-connect)
  - [How to connect to SMTP server](#how-to-connect-to-smtp-server)
  - [How to connect to email provider API](#how-to-connect-to-email-provider-api)
  - [How to connect to postgres database](#how-to-connect-to-postgres-database)
- [Integrations](#integrations)
  - [Email protocols](#email-protocols)
  - [Third-party APIs](#third-party-apis)
  - [Data sources](#data-sources)
- [Comparison with Mailchimp, Sendgrid, and ConvertKit](#comparison-with-mailchimp-sendgrid-and-convertkit)
- [Testing](#testing)

## Install Pigeon

### Install Pigeon from [crates.io](https://crates.io/crates/pigeon-rs)

``` bash
# Build and install pigeon binary to ~/.cargo/bin
cargo install pigeon-rs
```

_Note:_ Run `cargo install pigeon-rs` again to update to the latest version. Uninstall the pigeon binary with `cargo uninstall pigeon-rs`.

### Install Pigeon from [github.com](https://github.com/quambene/pigeon-rs)

``` bash
# Clone repository
git clone git@github.com:quambene/pigeon-rs.git
cd pigeon-rs

# Build and install pigeon binary to ~/.cargo/bin
cargo install --path .
```

_Note:_ Add `$HOME/.cargo/bin` to your `PATH` if it is missing:

``` bash
export PATH="$HOME/.cargo/bin:$PATH"
```

## Getting help

For getting help, try one of the following:

``` bash
# Check version
pigeon --version

# Print help
pigeon --help

# Print help for subcommand
pigeon help send
pigeon help send-bulk
pigeon help connect
pigeon help init
pigeon help query
pigeon help simple-query
pigeon help read
```

## Usage

Check connection to your SMTP server with `pigeon connect`:

``` bash
pigeon connect
```

> Connecting to SMTP server 'email-smtp.eu-west-1.amazonaws.com' ... <span style="color:MediumSeaGreen">ok</span>

See currently supported [integrations](#integrations) and [how to connect](#how-to-connect) below.

_Note:_ You can also check connection to third-party APIs instead of using the SMTP protocol. For example, using AWS Simple Email Service (SES): `pigeon connect aws`.

### Send email to a single receiver

Send a single email with subject and content:

``` bash
pigeon send \
    sender@your-domain.com \
    receiver@gmail.com \
    --subject "Test subject" \
    --content "This is a test email."
```

Send a single email with message defined in separate template file:

``` bash
pigeon send \
    sender@your-domain.com \
    receiver@gmail.com \
    --message-file "message.yaml"
```

The message template `message.yaml` is created with subcommand `init`:

``` bash
pigeon init
```

_Note:_ One of the advantages of a `--message-file` is that you can also draft the html version of your email. In contrast, with the options `--subject` and `--content` the email will only be sent in plaintext format.

If you prefer a dedicated HTML file for drafting your email, use the following command:

``` bash
pigeon send \
    sender@your-domain.com \
    receiver@gmail.com \
    --subject "Test subject" \
    --text-file "./message.txt" \
    --html-file "./message.html"
```

where `--text-file` defines the plaintext and `--html-file` the HTML version of your email.

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

Now send your newsletter to the queried receivers. If the table column name is different to "email" use `--receiver-column` to define a different column name. Let's try a `--dry-run` without confirmation `--assume-yes` first:

``` bash
pigeon send-bulk \
    albert@einstein.com \
    --receiver-query "select email from user where newsletter_confirmed = true" \
    --message-file "message.yaml" \
    --assume-yes \
    --dry-run
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
pigeon send-bulk \
    albert@einstein.com \
    --receiver-query "select first_name, last_name, email from user where newsletter_confirmed = true" \
    --message-file "message.yaml" \
    --personalize "first_name" "last_name" \
    --display
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

### How to connect to SMTP server

To connect to a SMTP server, define environment variables `SMTP_SERVER`, `SMTP_USERNAME`, and `SMTP_PASSWORD`. For example, using AWS SES:

``` bash
SMTP_SERVER=email-smtp.eu-west-1.amazonaws.com
SMTP_USERNAME=...
SMTP_PASSWORD=...
```

Source your environment `.env` in your current shell:

``` bash
set -a && source .env && set +a
```

### How to connect to email provider API

Instead of using SMTP, you can send emails via the API of a specific email provider as well.

Using AWS SES, define the following environment variables:

``` bash
AWS_ACCESS_KEY_ID=...
AWS_SECRET_ACCESS_KEY=...
AWS_REGION=eu-west-1
```

where `AWS_REGION` depends on the specified region for your AWS SES account.

Source your environment again:

``` bash
set -a && source .env && set +a
```

Send an email using `--connection`:

``` rust
pigeon send \
    sender@your-domain.com \
    receiver@gmail.com \
    --connection aws \
    --message-file "message.yaml"
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

_CAUTION:_ Connecting via TLS is not supported yet. Forward a local port through a SSH tunnel instead, e.g.:

``` bash
pigeon query "select email from user where newsletter_confirmed = true" --display --ssh-tunnel 5437
```

In addition to the environment variables above, `SERVER_USER` and `SERVER_HOST` have to be set for the SSH connection (`ssh user@host`).

## Integrations

### Email protocols

- MIME
- SMTP

### Third-party APIs

- AWS SES

### Data sources

- PostgreSQL
- CSV

## Comparison with Mailchimp, Sendgrid, and ConvertKit

These numbers may be outdated. Do your own research.

The following table compares the price per month for email provider and emails per month.

&nbsp; | 5,000 | 10,000 | 100,000
--- | --- | --- | ---
**Pigeon+**[**AWS**](https://aws.amazon.com/ses/pricing/) | $4.50 | $5 | $14
[**Mailchimp Marketing**](https://mailchimp.com/pricing/marketing/) | $9.99 | $20.99 | $78.99
[**Mailchimp Transactional**](https://mailchimp.com/pricing/transactional-email/) | - | - | $80
[**Sendgrid Marketing**](https://sendgrid.com/pricing/) | $15 | $15 | $120
[**Sendgrid API**](https://sendgrid.com/pricing/) | $14.95 | $14.95 | $29.95
[**ConvertKit**](https://convertkit.com/pricing) | $66 | $100 | $516

The following table shows the daily limit for sent emails per provider.

provider | daily limit
--------- | ---------
Pigeon+AWS | 50,000
Mailchimp | equals monthly limit
Sendgrid | equals monthly limit

## Testing

Some integration tests require a locally running database, and an AWS SES
account:

1. Specify the following environment variables:
   - SMTP
     - `SMTP_SERVER`
     - `SMTP_USERNAME`
     - `SMTP_PASSWORD`
   - AWS SES
     - `AWS_ACCESS_KEY_ID`
     - `AWS_SECRET_ACCESS_KEY`
     - `AWS_REGION`
   - Postgres
     - `DB_HOST`
     - `DB_PORT`
     - `DB_USER`
     - `DB_PASSWORD`
     - `DB_NAME`
2. Set up a temporary postgres db: `docker-compose run --rm --service-ports postgres`

``` bash
# Run unit tests and integration tests
cargo test

# Run unit tests
cargo test --lib

# Run integration tests
cargo test --test '*'
```
