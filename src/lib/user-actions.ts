import type { Prisma } from "@prisma/client";
import { db } from "./db";
import { generateSecret } from "../util";

export async function insertUserAction(
  userAction: Omit<
    Prisma.XOR<
      Prisma.UserActionsCreateInput,
      Prisma.UserActionsUncheckedCreateInput
    >,
    "secret"
  >
) {
  return await db.userActions.create({
    data: {
      secret: generateSecret(),
      ...userAction,
    },
  });
}
