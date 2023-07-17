# indent-war-crime

Usage: `indent-war-crime <dir-to-exec>`
Change this
```java
    private static Solid[] getSplit(Shape shape) {
        return splitMap.computeIfAbsent(shape, (s) -> {
            List<BoundingBox> bounds = shape.childBounds();

            List<Bounds3D> lower = new ArrayList<>(bounds.size());
            List<Bounds3D> upper = new ArrayList<>(bounds.size());

            for (BoundingBox boundingBox : bounds) {
                if (boundingBox.maxY() > 1) {
                    double minY = boundingBox.minY();
                    upper.add(Bounds3D.immutable(boundingBox.minX(), 1, boundingBox.minZ(), boundingBox.width(),
                            boundingBox.height() - 1, boundingBox.depth()));

                    if (minY < 1) {
                        lower.add(Bounds3D.immutable(boundingBox.minX(), minY, boundingBox.minZ(), boundingBox.width(),
                                1 - minY, boundingBox.depth()));
                    }
                } else {
                    lower.add(Bounds3D.immutable(boundingBox.minX(), boundingBox.minY(), boundingBox.minZ(),
                            boundingBox.width(), boundingBox.height(), boundingBox.depth()));
                }
            }

            return new Solid[] { Solid.of(lower.toArray(Bounds3D[]::new)), Solid.of(upper.toArray(Bounds3D[]::new)) };
        });
    }

    private static Solid cachedSolid(Shape shape) {
        return shapeMap.computeIfAbsent(shape, key -> {
            List<BoundingBox> boundingBoxes = key.childBounds();
            Bounds3D[] bounds = new Bounds3D[boundingBoxes.size()];

            for (int i = 0; i < bounds.length; i++) {
                BoundingBox box = boundingBoxes.get(i);
                bounds[i] = Bounds3D.immutable(box.minX(), box.minY(), box.minZ(), box.width(), box.height(),
                        box.depth());
            }

            return Solid.of(bounds);
        });
    }
}
```

to this
```java
                            private static Solid[] getSplit(Shape shape) {
                        return splitMap.computeIfAbsent(shape, (s) -> {
                    List<BoundingBox> bounds = shape.childBounds();
                                
                    List<Bounds3D> lower = new ArrayList<>(bounds.size());
                    List<Bounds3D> upper = new ArrayList<>(bounds.size());
                                
                    for (BoundingBox boundingBox : bounds) {
                if (boundingBox.maxY() > 1) {
            double minY = boundingBox.minY();
            upper.add(Bounds3D.immutable(boundingBox.minX(), 1, boundingBox.minZ(), boundingBox.width(),
    boundingBox.height() - 1, boundingBox.depth()));
                                
            if (minY < 1) {
        lower.add(Bounds3D.immutable(boundingBox.minX(), minY, boundingBox.minZ(), boundingBox.width(),
1 - minY, boundingBox.depth()));
            }
                } else {
            lower.add(Bounds3D.immutable(boundingBox.minX(), boundingBox.minY(), boundingBox.minZ(),
    boundingBox.width(), boundingBox.height(), boundingBox.depth()));
                }
                    }
                                
                    return new Solid[] { Solid.of(lower.toArray(Bounds3D[]::new)), Solid.of(upper.toArray(Bounds3D[]::new)) };
                        });
                            }
                                
                            private static Solid cachedSolid(Shape shape) {
                        return shapeMap.computeIfAbsent(shape, key -> {
                    List<BoundingBox> boundingBoxes = key.childBounds();
                    Bounds3D[] bounds = new Bounds3D[boundingBoxes.size()];
                                
                    for (int i = 0; i < bounds.length; i++) {
                BoundingBox box = boundingBoxes.get(i);
                bounds[i] = Bounds3D.immutable(box.minX(), box.minY(), box.minZ(), box.width(), box.height(),
        box.depth());
                    }
                                
                    return Solid.of(bounds);
                        });
                            }
                                }
                                
```

Have fun using this in a pull request!
