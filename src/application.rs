use std::cmp::PartialEq;
use std::fmt;
use std::net::Ipv4Addr;
use crate::fonts::{GSANSCODE_BOLD, ICON_ACCOUNT_TREE, ICON_LAN, icon, load_fonts};
use crate::theme::{button_style, container_style, text_input_style};
use iced::widget::{Container, column, container, pick_list, row, text, text_input, button, scrollable};
use iced::{Alignment, Font, Size};

#[derive(Debug, Clone)]
pub struct Subnet {
    pub network: Ipv4Addr,
    pub gateway: Ipv4Addr,
    pub broadcast: Ipv4Addr,
    pub usable_start: Ipv4Addr,
    pub usable_end: Ipv4Addr,
    pub prefix: u8,
    pub usable_hosts: u32,
}

#[derive(Debug, Clone)]
pub struct Application {
    pub ip_addr: String,
    pub cidr: String,
    pub subnetting_type: SubnettingType,
    pub flsm_length: String,
    pub results: Option<Vec<Subnet>>,
}

#[derive(Debug, Clone)]
pub enum Message {
    InputIpAddr(String),
    InputCidr(String),
    SubnettingType(SubnettingType),
    InputFlsmLength(String),
    Calculate
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum SubnettingType {
    FLSM,
    VLSM
}

impl fmt::Display for SubnettingType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FLSM => write!(f, "FLSM"),
            Self::VLSM => write!(f, "VLSM"),
        }
    }
}

impl SubnettingType {
    #[must_use]
    pub fn from(string: &str) -> Option<Self> {
        match string {
            "FLSM" => Some(Self::FLSM),
            "VLSM" => Some(Self::VLSM),
            _ => None,
        }
    }
}

impl Default for Application {
    fn default() -> Self {
        Self {
            ip_addr: String::default(),
            cidr: String::default(),
            subnetting_type: SubnettingType::FLSM,
            flsm_length: String::default(),
            results: None,
        }
    }
}

impl Application {
    #[must_use] 
    pub fn default_settings() -> iced::Settings {
        iced::Settings {
            antialiasing: true,
            default_font: Font::with_name("Google Sans Code"),
            fonts: load_fonts(),
            ..Default::default()
        }
    }

    #[must_use] 
    pub fn default_window() -> iced::window::Settings {
        iced::window::Settings {
            size: Size::new(800f32, 600f32),
            resizable: true,
            ..Default::default()
        }
    }

    fn calculate(&self) -> Option<Vec<Subnet>> {
        let ip: Ipv4Addr = self.ip_addr.parse().ok()?;
        let cidr: u8 = self.cidr.parse().ok()?;

        if cidr > 30 {
            return None;
        }

        let mask = u32::MAX << (32 - cidr);
        let network_u32 = u32::from(ip) & mask;
        let host_bits = 32 - cidr;

        match self.subnetting_type {
            SubnettingType::FLSM => {
                let num_subnets: u32 = self.flsm_length.parse().ok()?;
                if num_subnets == 0 || num_subnets > (1 << host_bits) {
                    return None;
                }
                let bits_needed = u8::try_from(num_subnets.next_power_of_two().trailing_zeros()).ok()?;
                let subnet_prefix = cidr + bits_needed;
                if subnet_prefix > 30 {
                    return None;
                }

                let subnet_bits = 32 - subnet_prefix;
                let subnet_size = 1u32 << subnet_bits;
                let mut subnets = Vec::new();

                for i in 0..num_subnets {
                    let subnet_start = network_u32 + (i * subnet_size);
                    let subnet_end = subnet_start + subnet_size - 1;
                    let gateway = subnet_start + 1;
                    let usable_start = subnet_start + 1;
                    let usable_end = subnet_end - 1;

                    subnets.push(Subnet {
                        network: Ipv4Addr::from(subnet_start),
                        gateway: Ipv4Addr::from(gateway),
                        broadcast: Ipv4Addr::from(subnet_end),
                        usable_start: Ipv4Addr::from(usable_start),
                        usable_end: Ipv4Addr::from(usable_end),
                        prefix: subnet_prefix,
                        usable_hosts: if subnet_bits > 1 { subnet_size - 2 } else { 0 },
                    });
                }

                Some(subnets)
            }
            SubnettingType::VLSM => Some(vec![]),
        }
    }
    
    /// # Panics
    /// Panics if panics
    pub fn update(&mut self, message: Message) {
        match message {
            Message::InputIpAddr(ip_addr) => {
                self.ip_addr = ip_addr;
            }
            Message::InputCidr(cidr) => {
                self.cidr = cidr;
            }
            Message::SubnettingType(subnetting_type) => {
                self.subnetting_type = subnetting_type;
            }
            Message::InputFlsmLength(length) => {
                self.flsm_length = length;
            }
            Message::Calculate => {
                self.results = self.calculate();
            }
        }
    }

    /// # Panics
    /// Panics if the selected subnetting type string cannot be parsed back to `SubnettingType`.
    pub fn view(&'_ self) -> Container<'_, Message> {
        let base_network = container(
            column![
                row![icon(ICON_LAN), text("Base network").font(GSANSCODE_BOLD)].spacing(8.0),
                row![
                    text_input("IP address", self.ip_addr.as_str())
                        .on_input(Message::InputIpAddr)
                        .width(192.0)
                        .style(text_input_style),
                    text("/"),
                    text_input("CIDR", self.cidr.as_str())
                        .on_input(Message::InputCidr)
                        .width(48.0)
                        .style(text_input_style),
                ]
                .align_y(Alignment::Center)
                .spacing(4.0)
            ]
            .spacing(9.0),
        )
        .padding(12.0)
        .style(|_| container_style());

        let flsm = container(
            column![
                text("Number of networks"),
                text_input("e.g. 8", self.flsm_length.as_str())
                    .on_input(Message::InputFlsmLength)
                    .width(72.0)
                    .style(text_input_style),
            ].spacing(8.0)
        );

        let subnetting_config = container(
            column![
                row![
                    icon(ICON_ACCOUNT_TREE),
                    text("Subnetting").font(GSANSCODE_BOLD),
                    container(
                        pick_list(
                            [SubnettingType::FLSM, SubnettingType::VLSM].map(|s| s.to_string()).to_vec(),
                            Some(self.subnetting_type.to_string()),
                            |s| Message::SubnettingType(SubnettingType::from(&s).expect("Invalid subnetting type"))
                    ))
                    .align_x(Alignment::End),
                ]
                .spacing(8.0)
                .align_y(Alignment::Center),

                if self.subnetting_type == SubnettingType::FLSM {
                    flsm
                }
                else {
                    container(
                        text!("Subnetting Type: {:?}", self.subnetting_type.to_string()),
                    )
                }
            ]
            .spacing(9.0),
        )
        .padding(12.0)
        .style(|_| container_style());

        container(
            column![
                base_network,
                subnetting_config,
                button("Calculate").on_press(Message::Calculate).style(button_style),
                if let Some(subnets) = &self.results {
                    let header = row![
                        text("Network").width(130.0),
                        text("Gateway").width(130.0),
                        text("Broadcast").width(130.0),
                        text("Start").width(130.0),
                        text("End").width(130.0),
                        text("Prefix").width(60.0),
                        text("Hosts").width(80.0),
                    ]
                    .spacing(12.0)
                    .padding(8.0);

                    let mut rows_col = column![header].spacing(4.0).padding(8.0);
                    for subnet in subnets {
                        rows_col = rows_col.push(
                            row![
                                text(subnet.network.to_string()).width(130.0),
                                text(subnet.gateway.to_string()).width(130.0),
                                text(subnet.broadcast.to_string()).width(130.0),
                                text(subnet.usable_start.to_string()).width(130.0),
                                text(subnet.usable_end.to_string()).width(130.0),
                                text(format!("/{}", subnet.prefix)).width(60.0),
                                text(subnet.usable_hosts.to_string()).width(80.0),
                            ]
                            .spacing(12.0)
                            .padding(8.0)
                        );
                    }

                    scrollable(
                        container(rows_col)
                            .style(|_| container_style())
                            .padding(8.0)
                    )
                    .height(250.0)
                } else {
                    scrollable(container(text("")))
                        .height(0.0)
                }
            ].spacing(12.0)
        ).padding(12)
    }
}
