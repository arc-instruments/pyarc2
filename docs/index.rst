.. pyarc2 documentation master file, created by
   sphinx-quickstart on Thu Jun  9 20:51:30 2022.

libarc2 Python bindings
=======================

``pyarc2`` provides Python bindings for the Rust libarc2_ library which is the
low-level library to access the ArC TWO parallel characterisation platform. It
exposes most of the underlying Rust functionality and allows users to build
complex programs that leverage the capabilities of ArC TWO. It is recommended
reading for new users otherwise you can head on straight to the :doc:`API
reference <api>`. Please note that ``pyarc2`` provides only a veneer around the
low-level Rust API and its target group is users wanting to develop tailored
characterisation tools on top of ArC TWO. For general testing usage it is
recommended that you get started with arc2control_ instead.


Installation
------------

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
   protocol

.. _libarc2: https://github.com/arc-instruments/libarc2
.. _beastlink: https://www.cesys.com/en/our-products/software-ip/beastlink.html
.. _`CESYS download page`: https://www.cesys.com/en/service-support/download-center/fpga.html
.. _`our repository`: https://github.com/arc-instruments/beastlink-rs/tree/master/contrib
.. _website: http://arc-instruments.co.uk/
.. _arc2control: https://github.com/arc-instruments/arc2control
