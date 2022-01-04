## Postgis

```sql
SELECT ST_Volume(geom) FROM 
(SELECT 
 	ST_3DDifference(a,b) FROM (
		SELECT 
		ST_3DUnion(
			ST_3DMakeBox(
				ST_MakePoint(0,0,0),
				ST_MakePoint(4,4,4)
			),
			ST_3DMakeBox(
				ST_MakePoint(2,2,2),
				ST_MakePoint(6,6,6)
			)
		) AS a,
		ST_3DMakeBox(
			ST_MakePoint(4,4,4),
			ST_MakePoint(5,5,5)
		) AS b
	) x) AS f(geom); 
```

fails - Postgis/SFCGAL can't compute volumes on solids with holes.