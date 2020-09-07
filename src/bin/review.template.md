---
category:       General Utility
description:    
msrv:           
---

# {{crate.name}}

...



## Audit

| version   | thoroughness | understanding | rating | notes |
| --------- | ------------ | ------------- | ------ | ----- |
{{#versions}}
| [{{{version}}}] | 
{{/versions}}

<!--
    thoroughness:   none low medium high
    understanding:  none low medium high
    rating:         dangerous negative neutral positive strong
-->

{{#versions}}
[{{{version}}}]: #{{{version}}}
{{/versions}}

{{#versions}}
<h2 name="{{{version}}}">{{version}}</h2>

| {{file_or_diff}}                                                      | Rating | Notes |
| --------------------------------------------------------- | ------ | ----- |
{{#files}}
| {{{name}}} | ✔️❔⚠️❗️
{{/files}}

{{/versions}}
| Other     | Rating | Notes |
| --------- | ------ | ----- |
| unsafe    | ✔️❔⚠️❗️
| fs        | ✔️❔⚠️❗️
| io        | ✔️❔⚠️❗️
| docs      | ✔️❔⚠️❗️
| tests     | ✔️❔⚠️❗️

<h2 name="0.0.1/src/lib.rs">src/lib.rs</h2>

| Line  | Notes |
| -----:| ----- |
| 


<!-- Templates

✔️❔⚠️❗️

#### :exclamation:  \[1\] Unsound ...
#### \[1\] Note ...
[1]: #exclamation--1-unsound-...
[2]: #1-note-...
[user/repository#1]: https://github.com/user/repository/issues/1
[user/repository#1]: https://github.com/user/repository/pull/1



# DiffVersionTemplate

| diff                  | rating | notes |
| --------------------- | ------ | ----- |
| 

# Full File Version Template

| Line  | Notes |
| -----:| ----- |
| 

-->
