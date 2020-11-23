﻿using System;
using System.Threading.Tasks;
using Buttplug;

namespace RemoteConnectorExample
{
    class Program
    {
        static async Task ConnectWebsocket()
        {
            // Creating a Websocket Connector is as easy as adding the
            // Client.Connectors.WebsocketConnector package from nuget and
            // passing a websocket address to it.
            var connector = new ButtplugWebsocketConnectorOptions(
                new Uri("ws://localhost:12345/buttplug").ToString());
            var client = new ButtplugClient("Example Client");
            await client.Connect(connector);
        }

        static void Main(string[] args)
        {
            ConnectWebsocket().Wait();
        }
    }
}
