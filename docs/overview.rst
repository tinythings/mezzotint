.. overview:

Overview
========

.. note:: **Abstract**

    What is a container and what is the difference between an app-bundle? How did we came to the point where our containers are as big as the operating systems for just one application?

.. toctree::
    :maxdepth: 1

    overview/workflow


Definitions
===========

What is a Container?
--------------------

A Linux container is a *lightweight*, isolated environment that contains a user-space operating system environment. This way it packages software with all its dependencies so that the application runs quickly and reliably from one computing environment to another.

What is an application bundle?
------------------------------

An application bundle is a way of organizing related resources, such as an application’s executable and its graphics, into a single directory that appears as a single file to the user. This makes it easier to distribute and install applications, as well as to manage their dependencies and updates.


Differences
===========

The following table roughly explains a typical difference between an application bundles and a containers:

Purpose
-------

An **Application Bundle**, like those on macOS, is a packaging format primarily used for organizing and distributing applications on a specific operating system (in this case, macOS). It contains all the necessary files and resources required to run the application on that particular OS.

A **Linux Container** is a lightweight, portable, and self-sufficient environment that encapsulates an application along with its dependencies, libraries, and runtime components. It enables an application to run consistently across various computing environments.

Isolation
---------

An **Application Bundle** doesn't inherently provide isolation. It's meant to organize files and resources but doesn't necessarily isolate the application from the rest of the system.

**Containers**, however, leverage kernel-level isolation features of the Linux operating system to create isolated environments. They offer a higher level of isolation compared to application bundles, allowing applications to run independently without interfering with other applications or the host system.

Portability
-----------

**Application Bundles** are primarily designed for specific operating systems. For instance, MacOS application bundles are specific to those systems and won't run natively on other operating systems without compatibility layers or emulation.

**Linux Container** are a bit more portable, in a way. Once created, a container can run on any system that supports the containerization technology (like Docker or Kubernetes), running a compatible kernel underneath. This means a containerized application can possibly run on various Linux distributions and even on other operating systems such as like Windows or `FreeBSD <https://productionwithscissors.run/2022/09/04/containerd-linux-on-freebsd/>`__.

Resource Sharing
----------------

Resources in an **Application Bundle** are contained within the bundle itself and are not typically shared with other applications.

**Containers** can share resources with the host system or other containers, depending on how they are configured. This sharing can include network resources, storage, or even the kernel, which allows for more efficient resource utilization.
In summary, an application bundle is a packaging format for organizing and distributing applications on a specific operating system, while a Linux container is a technology for creating lightweight, isolated environments that can run applications independently across different systems.

Goal
====
The goal of the Mezzotint is to combine the best of it into one container.