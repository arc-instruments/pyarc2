libarc2 Python bindings
=======================

.. important::
   If you recently acquired an ArC TWO it is recommended that you go through
   board's `📖 general documentation`_ first as it provides useful introductory
   information to get started with your instrument.

``pyarc2`` provides Python bindings for the Rust libarc2_ library which is the
low-level library to access the ArC TWO parallel characterisation platform. It
exposes most of the underlying Rust functionality and allows users to build
complex programs that leverage the capabilities of ArC TWO. It is recommended
reading for new users otherwise you can head on straight to the :doc:`API
reference <api>`. ``pyarc2`` provides a veneer around the low-level Rust API
and it is recommended for users wanting to develop tailored characterisation
tools on top of ArC TWO.


Installation
------------

Quick links: |pyarc2_github|_ |pyarc2_docs|_ |pyarc2_pypi|_

``pyarc2`` should be available from pip. For the stable version run

.. code-block:: console

   $ pip install pyarc2

or for the latest and greatest

.. code-block:: console

   $ pip install git+https://github.com/arc-instruments/pyarc2

In addition to ``pyarc2`` you will also need the necessary libusb driver to
interact with ArC TWO. ``pyarc2`` wheels will typically include the library
that implements the FPGA API (see beastlink_) but the actual USB driver should
be installed separately for your operating system. This is typically included
with the CESYS distribution of beastlink. Check `CESYS download page`_ for more
details. For Windows run the installer provided by CESYS. For Linux scripts to
generate suitable packages for Archlinux, Debian-based and RedHat compatible
distributions are available from `our repository`_.

``pyarc2`` is only available for Windows and glibc Linux x86_64.  That's due to
limitations of beastlink. Should more architectures and operating systems be
supported by beastlink these will be made available but it's not something
that's under control of the ``libarc2`` and ``pyarc2`` development team.


ArC TWO firmware
----------------

A firmware is required to talk to ArC TWO. This is not typically provided by
``pyarc2`` but it is available from our website_.  Compiled distributions of
higher-level applications, such as arc2control_ would typically also include
the latest firmware.


Contents of this guide
======================

.. toctree::
   :maxdepth: 2

   overview
   api

.. _libarc2: https://github.com/arc-instruments/libarc2
.. _beastlink: https://www.cesys.com/en/our-products/software-ip/beastlink.html
.. _`CESYS download page`: https://www.cesys.com/en/service-support/download-center/fpga.html
.. _`our repository`: https://github.com/arc-instruments/beastlink-rs/tree/master/contrib
.. _website: http://arc-instruments.co.uk/
.. _arc2control: https://github.com/arc-instruments/arc2control
.. _`📖 general documentation`: https://files.arc-instruments.co.uk/documents/arc2-general
.. |pyarc2_github| image:: https://img.shields.io/github/v/tag/arc-instruments/pyarc2?logo=github&label=%20
.. _pyarc2_github: https://github.com/arc-instruments/pyarc2
.. |pyarc2_docs| image:: https://img.shields.io/badge/-documentation-default
.. _pyarc2_docs: https://files.arc-instruments.co.uk/documents/pyarc2
.. |pyarc2_pypi| image:: https://img.shields.io/pypi/v/pyarc2?logo=python&logoColor=white
.. _pyarc2_pypi: https://pypi.org/project/pyarc2
