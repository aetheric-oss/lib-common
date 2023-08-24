![Arrow Banner](https://github.com/Arrow-air/tf-github/raw/main/src/templates/doc-banner-services.png)

# Concept of Operations - `lib-common`

## :telescope: Overview

Common library providing commonly used functionality for the Arrow Services.

### Metadata

| Attribute     | Description                                                       |
| ------------- |-------------------------------------------------------------------|
| Maintainer(s) | [Services Team](https://github.com/orgs/Arrow-air/teams/services) |
| Status        | Development                                                       |

## :books: Related Documents

Document | Description
--- | ---
[High-Level Concept of Operations (CONOPS)](https://github.com/Arrow-air/se-services/blob/develop/docs/conops.md) | Overview of Arrow microservices.

## :raised_hands: Motivation

The development of microservices-based applications has become increasingly popular due to its scalability, flexibility, and modularity.
However, as the number of microservices grows, managing common functionalities across these services becomes a challenge.
Therefore, the need for a library that provides generic functions that can be used across multiple microservices has become more critical.
This library will enable developers to save time and effort by providing them with a pre-built set of functions that can be easily integrated into their microservices, resulting in a more efficient and standardized development process.

## Needs, Goals and Objectives of Envisioned System

The 'lib-common' library is a Rust-based library designed to provide a set of common functionalities that can be used across different services or applications.
The library aims to reduce the development time and effort required to implement common functionalities such as logging, error handling, configuration management, and gRPC client creation, among others.
By providing a set of reusable and well-tested functions, the library aims to improve the consistency and reliability of software development while reducing the burden on developers to implement and maintain these functionalities.
The objectives of the library include providing high code quality and reliability, ensuring compatibility with different Rust versions, and supporting different architectures and design patterns.
The library does not have an API, but instead offers a set of Rust modules that can be included in applications as needed.
Ultimately, the envisioned system aims to improve the efficiency and consistency of software development by providing a reliable and reusable set of common functionalities that can be easily integrated into different types of services or applications.

## External Interfaces
See the ICD for this library.

## Physical Environment

See the High-Level CONOPS.

## Support Environment

See the High-Level CONOPS.

## Risks and Potential Issues

While the 'lib-common' library offers many benefits, there are also potential risks and issues associated with its use.

One potential risk is that the library may not be suitable for all applications or services, as different software may have unique requirements that the library does not address.
Additionally, the library may introduce new dependencies into the software, which can increase the complexity of the software and potentially introduce new vulnerabilities or conflicts with existing dependencies.
Moreover, there is a risk that the library's functions may not be reliable or well-tested, which can lead to errors or bugs in the software.
To mitigate these risks and potential issues, it is important to thoroughly evaluate the library's functions and test them in different software environments before incorporating them into a production application or service. Additionally, it is important to ensure that the library is kept up-to-date with the latest Rust releases and security patches to minimize the risk of vulnerabilities.

## Appendix A: Acronyms & Glossary

See [Arrow Glossary](https://www.arrowair.com/docs/documentation/glossary).
