SailStats Analyzer
==================

A visual analyzer for files logged with [SailStats Logger](https://github.com/setnorth/sailstats-logger), or any csv file adhering to the same format. 

SailStats Analyzer can give you an insight into your sailing performance based on the collected data from your on-board instruments.

Prequisites
-----------
1. To build the exectuable from source you need to have a functional tauri development environment installed on your computer. Visit [Tauri](https://tauri.studio/) for more information.
2. Run `git clone https://github.com/setnorth/sailstats-analyzer sailstats-analyzer`
3. Run `cd sailstats-analyzer && yarn install`

Building Executable
-------------------
To build an executable from the sources use `yarn tauri build`.

Development Setup
-----------------
Run `yarn run dev` (the serving process, that refreshes on code changes), then in another terminal `yarn tauri dev` (the actual window). This is right now split into two different commands since there seems to be an issue withing tauris with the `beforeDevCommand` option.

License
-------
SailStats Analyzer 0.1.0a
Copyright (C) 2021  Thorsten Ernst Schilling <thorsten.schilling@gmail.com>

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
