 SELECT barcodes.barcode,
    barcodes.fehler,
    barcodes.fehler_auswahl,
    (barcodes.created_at AT TIME ZONE 'Europe/Berlin'::text) AS created_at,
    "Up Users - User".username AS "Up Users - User__username"
   FROM ((barcodes
     LEFT JOIN barcodes_users_permissions_user_links "Barcodes Users Permissions User Links" ON ((barcodes.id = "Barcodes Users Permissions User Links".barcode_id)))
     LEFT JOIN up_users "Up Users - User" ON (("Barcodes Users Permissions User Links".user_id = "Up Users - User".id)))
  ORDER BY barcodes.created_at DESC;