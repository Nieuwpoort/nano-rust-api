use lettre::message::header::ContentType;
use lettre::message::{Mailbox, Message};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{AsyncSmtpTransport, Tokio1Executor, AsyncTransport};
use crate::structs::cache::env::ENV_CACHE;

pub async fn send_tps_warning_email() {
    let env = ENV_CACHE.get().unwrap();

    let html_body = format!(
        r#"
        <html>
            <body style="font-family: 'Inter', Arial, sans-serif; background: #f8fafc; color: #222; margin: 0; padding: 0;">
                <div style="max-width: 480px; margin: 40px auto; background: #fff; border-radius: 12px; box-shadow: 0 2px 8px rgba(0,0,0,0.07); padding: 32px;">
                    <h2 style="color: #1e293b; margin-bottom: 16px;">TPS Warning from <strong> x402</strong><strong style="color: #3b82f6;">Nano</strong></h2>
                    <p style="font-size: 16px; line-height: 1.5; margin-bottom: 24px;">
                        Dear Moderater,
                    </p>
                    <p style="font-size: 16px; line-height: 1.5; margin-bottom: 24px;">
                        We have detected that the Transactions Per Second (TPS) on our network has exceeded the warning threshold. This may impact the performance of our services.
                    </p>
                    <p style="font-size: 16px; line-height: 1.5; margin-bottom: 24px;">
                        Please monitor the situation and take necessary actions to ensure smooth operation.
                    </p>
                    <p style="font-size: 16px; line-height: 1.5; margin-bottom: 24px;">
                        Best regards,<br/>
                        The ifenpay Team
                    </p>
                </div>
            </body>
        </html>
        "#,
    );

    let creds: Credentials = Credentials::new(env.smtp_user.clone(), env.smtp_pass.clone());
    let mut builder = Message::builder()
    
        .from(Mailbox::new(Some("ifenpay".to_string()), env.smtp_user.parse().unwrap()))

        .subject("TPS Warning from ifenpay")
        .header(ContentType::TEXT_HTML);

    for recipient in env.alert_email_recipients.iter() {
        builder = builder.to(recipient.parse::<Mailbox>().unwrap());
    }

    let mail = builder.body(html_body).unwrap();

    let mailer = AsyncSmtpTransport::<Tokio1Executor>::relay(env.smtp_host.as_str()).unwrap()
        .port(env.smtp_port)
        .credentials(creds)
        .build();

    mailer.send(mail).await.unwrap();
}   

pub async fn send_tps_critical_email() {
    let env = ENV_CACHE.get().unwrap();

    let html_body = format!(
        r#"
        <html>
            <body style="font-family: 'Inter', Arial, sans-serif; background: #f8fafc; color: #222; margin: 0; padding: 0;">
                <div style="max-width: 480px; margin: 40px auto; background: #fff; border-radius: 12px; box-shadow: 0 2px 8px rgba(0,0,0,0.07); padding: 32px;">
                    <h2 style="color: #1e293b; margin-bottom: 16px;">TPS Critical from <strong> x402</strong><strong style="color: #3b82f6;">Nano</strong></h2>
                    <p style="font-size: 16px; line-height: 1.5; margin-bottom: 24px;">
                        Dear Moderater,
                    </p>
                    <p style="font-size: 16px; line-height: 1.5; margin-bottom: 24px;">
                        We have detected that the Transactions Per Second (TPS) on our network has exceeded the critical threshold. This may severely impact the performance of your transactions.
                    </p>
                    <p style="font-size: 16px; line-height: 1.5; margin-bottom: 24px;">
                        Please take immediate action to address this issue and ensure the stability of the network.
                    </p>
                    <p style="font-size: 16px; line-height: 1.5; margin-bottom: 24px;">
                        Best regards,<br/>
                        The ifenpay Team
                    </p>
                </div>
            </body>
        </html>
        "#,
    );
    let creds = Credentials::new(env.smtp_user.clone(), env.smtp_pass.clone());
    let mut builder = Message::builder()
        .from(Mailbox::new(Some("ifenpay".to_string()), env.smtp_user.parse().unwrap()))
        .subject("TPS Critical from ifenpay")
        .header(ContentType::TEXT_HTML);

    for recipient in env.alert_email_recipients.iter() {
        builder = builder.to(recipient.parse::<Mailbox>().unwrap());
    }

    let mail = builder.body(html_body).unwrap();

    let mailer = AsyncSmtpTransport::<Tokio1Executor>::relay(env.smtp_host.as_str()).unwrap()
        .port(env.smtp_port)
        .credentials(creds)
        .build();

    mailer.send(mail).await.unwrap();
}   
