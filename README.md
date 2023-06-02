
# cmasfo-git

The Main Repo

## Introduction

Basically this repo contains all other repos as submodules.

This repo contains lots of code, docs, or projects.

To see more information about cmasfo itself, please see docs directory.

The docs directory is also docs submodule.

I'm aware of that 'docs' folder does some special role in github,
but I'm not using it in that way.

And that role can be replaced by '.github' directory.
That's the way that I prefer.

## How to Clone This Repo

Prerequisites: git  
Download link: https://git-scm.com

```
git clone https://github.com/cmasfo/cmasfo-git
cd cmasfo-git
git submodule init
git submodule update
```

I do not prefer using submodules in hierarchical structure.

So there is no need to init or update recursively, at least for now.

## The Directory Structure

* .github
  * archive submodule
    * (works as trash bin)
  * profile submodule
    * README.md - github profile
  * [markdown files] (contribution, license)
  * [github components] (workflow, workspace)
* .vscode
  * [json files] (extensions, settings)
* [n_cmasfo-xxx]
  * cmasfo dev crate
  * [cmasfo production projects]
* [_xxx]
  * (main rust project components)
* [xxx]
  * (topic submodules)
* .gitignore
  * defines what to git-ignore
* .gitsubmodules
  * define git submodules
* Cargo.lock (ignored)
  * (rust cargo lock file)
* Cargo.toml
  * (main rust project manifest)
* README.md
  * (main repo readme file)
* build.rs
  * (defines rust build process)

## Naming or Coding Convention

* basically follow rust convention style
  * but rustfmt.toml can overwrite it
* file first line & last line
  * empty line
* indentation
  * 2 spaces
* brace {
  * } K & R
* snake & hyphen
  * firstlabel-lastlabel_firstname-lastname
* date & time
  * 20230421-172401 (yyyymmdd-hhmmss)
* end of brace
  * write label (no parameter)
```
fn func() {
  ...
} // fn func
```
* long comment
  * title & content
```
/*
  how to use this function

  Lorem ipsum ...
  ... lorem ipsum.

  Lorem ipsum ...
  ... lorem ipsum.
*/
```
* dir or file naming (ordering: unicode)
  * .dir (dot: hidden, based on unix-style)
  * 0_dir (number: structured, can be labeled with number)
  * Dir (upper-case: not component)
  * _dir (underscore: component, but with higher priority)
  * dir (lower-case: component)
  * NON-ENGLISH (non-english)
