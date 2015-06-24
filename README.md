# Citadel
Citadel is a tiered general-purpose library for Rust. It provides abstractions that I use in some of my other projects, so I can keep pulling citadel in and not having to duplicate effort.

I hope that you find these libraries as useful as I do.

# Installing
Simple.
````
    [dependencies.citadel]
    git = "https://www.github.com/natepisarski/citadel"
````

## Layout
Citadel has a pyramidal structure. The top-level libraries (currently just Prelude) are available to the entire framework. Each "Wing" of citadel (currently just access and modify) will only have access to the tier equal to or above it. Sub-tiers within these wings follow the same rules.

## Current Libraries
Here's a rundown of the current facilities citadel provides.

### Access Wing
Functions relating to the retrieval of information of slices / lists of information

 * stat - Metadata / Subsequencing
 * transform - List transformations
 * verify - Sanitization

### Modify Wing
Functions relating to the manipulation of lists and coercions of lists into other data types.
* coerce - Coercions of lists into other types of data.

## License 
Copyright (c) 2015, Nathaniel Pisarski
All rights reserved.

Redistribution and use in source and binary forms, with or without modification, are permitted provided that the following conditions are met:

1. Redistributions of source code must retain the above copyright notice, this list of conditions and the following disclaimer.

2. Redistributions in binary form must reproduce the above copyright notice, this list of conditions and the following disclaimer in the documentation and/or other materials provided with the distribution.

3. Neither the name of the copyright holder nor the names of its contributors may be used to endorse or promote products derived from this software without specific prior written permission.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
