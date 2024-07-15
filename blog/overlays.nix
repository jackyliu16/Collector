{ inputs, ... }: final: prev: {
  zola-with-ch-index = prev.zola.overrideAttrs (_: {
    cargoBuildFeatures = [ "indexing-zh" ];
  });
}
