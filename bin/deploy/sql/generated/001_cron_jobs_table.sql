IF OBJECT_ID(N'cronDefTable', N'U') IS NULL
BEGIN
  CREATE TABLE cronDefTable (
    Id uniqueidentifier NOT NULL DEFAULT NEWID() PRIMARY KEY,
    CreatedBy nvarchar(64) NOT NULL DEFAULT N'system',
    CreateAt datetime2 NOT NULL DEFAULT SYSUTCDATETIME(),
    UpdatedBy nvarchar(64) NOT NULL DEFAULT N'system',
    UpdateAt datetime2 NOT NULL DEFAULT SYSUTCDATETIME(),
    Name nvarchar(200) NOT NULL DEFAULT N'default-job',
    Host nvarchar(128) NOT NULL DEFAULT N'default',
    CronExpr nvarchar(120) NOT NULL,
    [Type] nvarchar(32) NOT NULL DEFAULT N'shell',
    Content nvarchar(max) NULL,
    Secret nvarchar(255) NULL,
    IsEnable bit NOT NULL DEFAULT 1,
    Concurrent bit NOT NULL DEFAULT 0,
    DelayStart int NOT NULL DEFAULT 0
  );
END;