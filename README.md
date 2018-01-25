# sssunpack

This tool allows the data files in Lunar: Silver Star Story Complete for the Saturn to be unpacked.

The original release of Lunar for the Saturn stores its data in a standard filesystem layout. The MPEG rerelease, however, packs most of its files into data files that have to be unpacked to look at individual files.

## Installation

On Mac:

> brew install studio-lucia/lunar/sssunpack

Building manually:

Clone this repo, and then

> cargo build

## Usage

sssunpack file.dat [directory_to_unpack_to]
