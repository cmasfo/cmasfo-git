
# README.md

Since '.github/README.md' overwrites '/README.md', I had to name this file '_README.md'.

It overwrites repo readme in github. If it's special repo, it also overwrites profile.

It's impossible to use different readme file for profile and special repo.

Thus, I resolve that with using profile repo separately.

For example, my special repo is 'cmasfo' but my main repo is 'cmasfo-git', which is different.

My preference is making profile repo as submodule, with path '.github/profile'.

It is same path to make repo readme in github organization's repository.

## README priority based on Directory

* .github/README.md (top priority)
* README.md
* docs/README.md

## README priority based on File Extension

* README.md
* README.rst
* README.txt
