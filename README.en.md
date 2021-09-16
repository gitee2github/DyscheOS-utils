# DyscheOS-utils

#### Description
This repo is about misc tools regard to userspace part of the Dysche solution.

Note, the master branch may not contain latest dev process. See the list of branches
for more details.

#### Software Architecture
The overall architecture of Dysche is originated from Linux AMP(Asychronous Multi-Processing) architecture.
In general, there are two parts in Dysche arch: Linux kernel part and the Linux userspace part. In this
repo, we'll focus on userspace part.

For Dysche userspace design, there are several componets logically:

- A tool to handle APP-OS images, load the image and verify the image etc.

- Interface with the Dysche kernel part, run the APP-OS.

- A system service to interfere with Dysche kernel part, maintain/operate APP-OS on the fly, provide
devices emulation etc.

#### Instructions

Refer to README.md file of each sub-dir in this repo please.

#### Contribution

1.  Fork the repository
2.  Create a branch
3.  Commit your changes
4.  Create Pull Request


