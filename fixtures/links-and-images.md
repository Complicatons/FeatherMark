# Links and images fixture

This file checks safe link routing and relative image resolution without relying on the network.

## Links

- [Jump to the target heading](#target-heading)
- [Open a secure website](https://example.com/feathermark)
- [Compose an email](mailto:reader@example.com?subject=FeatherMark%20fixture)
- [Refer to another local fixture](sample.md)

The fragment link should scroll inside FeatherMark. HTTPS and email links should be delegated to the operating system. The relative Markdown link should open in the current tab, while Ctrl+Click should open it in another tab.

## Relative images

Ordinary relative path:

![Small FeatherMark test image](images/feather-test.png)

Percent-encoded space in a relative path:

![The same test image from a filename containing a space](images/feather%20mark.png)

Both images should render. Neither path should grant access outside this fixture directory.

## Missing image

![Missing image with useful alternative text](images/does-not-exist.png)

The viewer should show a compact unavailable-image placeholder and remain responsive.

## Target heading {#target-heading}

If this heading is near the top of the window after following the fragment link, in-document navigation worked.
