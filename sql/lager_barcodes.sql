SELECT
  "public"."barcodes"."id" AS "Id",
  "public"."barcodes"."barcode" AS "Barcode",
  "Up Users - User"."username" AS "Mitarbeiter",
  "public"."barcodes"."fehler" AS "Fehler",
  "public"."barcodes"."fehler_auswahl" AS "Fehler Auswahl",
  "public"."barcodes"."created_at" AS "Datum"
FROM "public"."barcodes"
LEFT JOIN "public"."barcodes_users_permissions_user_links" "Barcodes Users Permissions User Links"
  ON "public"."barcodes"."id" = "Barcodes Users Permissions User Links"."barcode_id"
LEFT JOIN "public"."up_users" "Up Users - User"
  ON "Barcodes Users Permissions User Links"."user_id" = "Up Users - User"."id"
WHERE "Up Users - User"."rolle" = 'Lager'
UNION
SELECT
  "public"."barcodes"."id" AS "Id",
  "public"."barcodes"."barcode" AS "Barcode",
  "Up Users - User2"."username" AS "Mitarbeiter",
  "public"."barcodes"."fehler" AS "Fehler",
  "public"."barcodes"."fehler_auswahl" AS "Fehler Auswahl",
  "public"."barcodes"."created_at" AS "Datum"
FROM "public"."barcodes"
LEFT JOIN "public"."barcodes_lager_mitarbeiter_links" "Barcodes Lager Mitarbeiter Links"
  ON "public"."barcodes"."id" = "Barcodes Lager Mitarbeiter Links"."barcode_id"
LEFT JOIN "public"."up_users" "Up Users - User2"
  ON "Barcodes Lager Mitarbeiter Links"."user_id" = "Up Users - User2"."id"
WHERE "Up Users - User2"."rolle" = 'Lager'
ORDER BY "Id" DESC;