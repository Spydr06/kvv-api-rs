use kvv_efa_api::{self, request::{DepartureMonitorRequest, Request, StopFinderRequest}};

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {        
    {
        // Search the station table for "Hauptbahnhof"
        let request = StopFinderRequest::builder()
            .name("Bretten Bahnhof")
            .build();

        println!("Requesting {}...", request.url());

        let response = request.get().await?;

        println!("response: {:#?}", response);
    }

    {
        // Fetch the departures of station 7000801 ("Durlach Bahnhof")
        let request = DepartureMonitorRequest::builder()
            .name(7000801)
            .build();

        println!("Requesting {}...", request.url());

        let response = request.get().await?;

        println!("response: {:#?}", response);
    }

    Ok(())
}
