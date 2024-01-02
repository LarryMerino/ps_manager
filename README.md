# Prestashop invoice generator

## Description

This Rust-based project is designed to interact with the PrestaShop Webservice API to retrieve invoice data and generate PDF invoices locally from this data.

## Table of Contents
1. [Deployment](#deployment)
2. [Usage](#usage)
3. [Contribution](#contribution)
4. [License](#license)

## Deployment

To install and run this project, follow these steps:

1. Clone the repository to your local machine: ` git clone https://github.com/LarryMerino/ps-invoice-generator.git`
2. Run `docker compose up` command to start a database container and a Prestashop container. Data that requires persistence is stored in the `dev-files` directory
3. Establish a new store via a web browser by accessing the `localhost:8080` URL and following the Prestashop installation guidelines.

## Usage

Todo!

## Contribution

If you wish to contribute to this project, please follow these steps:

1. Fork the repository.
2. Make sure you are in the "develop" branch: `git checkout develop`
3. Create a new branch for your contribution: `git checkout -b my-contribution`
4. Make your changes and commit them: `git commit -m "Added feature XYZ"`
5. Push your changes to your fork: `git push origin my-contribution`
6. Create a pull request to the develop repository.

## License

This project is licensed under the "Apache License 2.0". See the [LICENSE](LICENSE) file for more details.