SELECT
  "public"."barcodes"."id" AS "Barcode",
  "public"."barcodes"."barcode" AS "bc",
  "Up Users - User"."username" AS "User",
  "public"."barcodes"."fehler" AS "fehler",
  "public"."barcodes"."fehler_auswahl" AS "fehler_auswahl",
  "public"."barcodes"."created_at" AS "created_at"
FROM "public"."barcodes"
LEFT JOIN "public"."barcodes_users_permissions_user_links" "Barcodes Users Permissions User Links"
  ON "public"."barcodes"."id" = "Barcodes Users Permissions User Links"."barcode_id"
LEFT JOIN "public"."up_users" "Up Users - User"
  ON "Barcodes Users Permissions User Links"."user_id" = "Up Users - User"."id"
WHERE "Up Users - User"."rolle" = 'Lager'
UNION
SELECT
  "public"."barcodes"."id" AS "Barcode",
  "public"."barcodes"."barcode" AS "bc",
  "Up Users - User2"."username" AS "User",
  "public"."barcodes"."fehler" AS "fehler",
  "public"."barcodes"."fehler_auswahl" AS "fehler_auswahl",
  "public"."barcodes"."created_at" AS "created_at"
FROM "public"."barcodes"
LEFT JOIN "public"."barcodes_lager_mitarbeiter_links" "Barcodes Lager Mitarbeiter Links"
  ON "public"."barcodes"."id" = "Barcodes Lager Mitarbeiter Links"."barcode_id"
LEFT JOIN "public"."up_users" "Up Users - User2"
  ON "Barcodes Lager Mitarbeiter Links"."user_id" = "Up Users - User2"."id"
WHERE "Up Users - User2"."rolle" = 'Lager'
ORDER BY "Barcode" DESC;