IF OBJECT_ID(N'cronBatchJobsTable', N'U') IS NULL
BEGIN
  CREATE TABLE cronBatchJobsTable (
    Id uniqueidentifier NOT NULL DEFAULT NEWID() PRIMARY KEY,
    CreatedBy nvarchar(64) NOT NULL DEFAULT N'system',
    CreateAt datetime2 NOT NULL DEFAULT SYSUTCDATETIME(),
    UpdatedBy nvarchar(64) NOT NULL DEFAULT N'system',
    UpdateAt datetime2 NOT NULL DEFAULT SYSUTCDATETIME(),
    Name nvarchar(200) NOT NULL DEFAULT N'default-batch-job',
    JobId uniqueidentifier NULL,
    Host nvarchar(128) NOT NULL DEFAULT N'default',
    StartDateTime datetime2 NULL,
    IsStarted bit NULL,
    IsEnable bit NOT NULL DEFAULT 1,
    [Once] bit NOT NULL DEFAULT 0,
    WorkBeginAt nvarchar(64) NULL,
    WorkEndAt nvarchar(64) NULL,
    Concurrent bit NOT NULL DEFAULT 0,
    DelayStart int NOT NULL DEFAULT 0,
    Content nvarchar(max) NULL,
    Result nvarchar(max) NULL,
    Secret nvarchar(255) NULL,
    [Type] nvarchar(32) NOT NULL DEFAULT N'shell'
  );
END;