-- CreateEnum
CREATE TYPE "UserActionType" AS ENUM ('CodeExecution');

-- CreateTable
CREATE TABLE "UserActions" (
    "id" SERIAL NOT NULL,
    "user_id" TEXT NOT NULL,
    "secret" TEXT NOT NULL,
    "type" "UserActionType" NOT NULL,
    "code_language" TEXT,
    "code_expression" TEXT,
    "code_output" TEXT,

    CONSTRAINT "UserActions_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "LinkedAIToken" (
    "user_id" TEXT NOT NULL,
    "ai_token" TEXT NOT NULL,

    CONSTRAINT "LinkedAIToken_pkey" PRIMARY KEY ("user_id")
);
