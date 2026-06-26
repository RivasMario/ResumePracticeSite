use leptos::prelude::*;

// Pull the reusable components from the components module.
use crate::components::{ExperienceItem, GithubRepos, Header, Section};

// The root component. Holds the resume data and lays out the page.
#[component]
pub fn App() -> impl IntoView {
    view! {
        <main class="page">
            <Header/>

            <Section title="Summary">
                <p>
                    "Cloud / SRE engineer with 9+ years sustaining Azure Government "
                    "infrastructure at 99.99% uptime. Active TS/SCI. Depth in IAM "
                    "microservices, CI/CD automation, and Infrastructure as Code across "
                    "air-gapped government clouds; prior Navy intelligence analyst. "
                    "Based in the Pacific Northwest."
                </p>
            </Section>

            <Section title="Experience">
                <ExperienceItem
                    role="Cloud Support Engineer 2"
                    company="ASM Research (Microsoft Vendor)"
                    dates="Dec 2018 \u{2014} Present"
                    bullets=vec![
                        "Sustained 99.99% uptime across 4+ Azure Government clouds supporting critical federal operations.",
                        "Shipped 100+ pull requests to the MSODS org repo (deployment scripts, ARM Ev2 configs) through code review, merge, and automated test pipelines.",
                        "Built VSRM ring-deployment pipelines in Azure DevOps to roll patches, microservice versions, and tool configs across Gov Clouds, reducing release risk via staged rings.",
                        "Operated Azure IAM microservices \u{2014} Service Fabric replicas (bare metal), AKS, and Cosmos DB \u{2014} at 99.99% availability for government tenants.",
                        "Instrumented logging and monitoring in Kusto and Geneva to detect and pre-empt outages before customer impact.",
                        "Imaged and patched hundreds of Windows Servers via WDS and SCCM/MECM with federally mandated cipher suites.",
                    ]
                />
                <ExperienceItem
                    role="Service Engineer"
                    company="Avanade (Microsoft Vendor)"
                    dates="Aug 2017 \u{2014} Dec 2018"
                    bullets=vec![
                        "Deployed Kubernetes v1.21 clusters on Azure IaaS (Linux), cutting infrastructure cost 25%; automated provisioning with Ansible v2.10 for +15% management savings.",
                        "Led DevSecOps adoption (IaC, key rotation, config management), improving system reliability and uptime 20%.",
                        "Hardened access with Azure Key Vault, NSG rules, RBAC, and IAM, resolving 250+ client security requirements annually.",
                        "Built a custom CI/CD pipeline (file management, Docker builds, Container Registry) that cut manual intervention 30% and release time 25%.",
                        "Engineered resilient cloud networks (VMs, VMSS, VNETs, load balancers) with auto-scaling.",
                    ]
                />
                <ExperienceItem
                    role="Intelligence Specialist / Expeditionary Warfare Analyst"
                    company="US Navy Reserves"
                    dates="2018 \u{2014} 2024 (Reserve)"
                    bullets=vec![
                        "Served as maritime, special-forces, and counterintelligence analyst at Combined Naval Forces Korea and Naval Special Warfare Group 2 (Pacific theater).",
                        "Exploited drone Full Motion Video (FMV) and GEOINT in Google Earth and ArcGIS to produce threat assessments.",
                        "Briefed daily Red Force and maritime intelligence to the US Korea J2 and delivered pre-mission briefs to Special Boat Team commanders.",
                    ]
                />
                <ExperienceItem
                    role="System Administrator"
                    company="Leidos (Microsoft Vendor)"
                    dates="Feb 2017 \u{2014} Aug 2017"
                    bullets=vec![
                        "Administered infrastructure for 1,000 users (Domain Controllers, Exchange, Active Directory), sustaining 98% satisfaction.",
                        "Resolved 120+ tickets monthly; managed PKI digital certificates.",
                    ]
                />
                <ExperienceItem
                    role="Service Engineer Intern"
                    company="Microsoft"
                    dates="Aug 2016 \u{2014} Feb 2017"
                    bullets=vec![
                        "Assessed compliance tooling for the Windows Devices group, identifying 50% potential savings and driving adoption of an external vendor solution.",
                    ]
                />
            </Section>

            <Section title="Projects">
                <ExperienceItem
                    role="Secure Proxmox Homelab"
                    company="Personal"
                    dates=""
                    bullets=vec![
                        "Entra ID SSO via an OAuth2 proxy on Ubuntu for centralized authentication and access control.",
                    ]
                />
                <ExperienceItem
                    role="PowerShell Automation"
                    company="Personal"
                    dates=""
                    bullets=vec![
                        "Two GUI tools: cut replica-server OS migration 6h\u{2192}4h (33%) and user error 35%.",
                        "Cloud-agnostic scanner (bare metal, chassis managers, VM inventories) for accurate service deconfliction.",
                    ]
                />
            </Section>

            <Section title="GitHub">
                <p class="section-note">"Live from the GitHub API \u{2014} most recently updated repos."</p>
                <GithubRepos/>
            </Section>

            <Section title="Certifications">
                <ul class="skills">
                    {["CompTIA Security+", "Azure Administrator (AZ-104)", "Azure Fundamentals (AZ-900)", "LPIC-1"]
                        .into_iter()
                        .map(|c| view! { <li>{c}</li> })
                        .collect_view()}
                </ul>
            </Section>

            <Section title="Skills">
                <ul class="skills">
                    {["Azure (Government Cloud)", "Site Reliability / SRE", "Kubernetes / AKS",
                      "Service Fabric", "Cosmos DB", "Azure DevOps", "CI/CD",
                      "Infrastructure as Code (ARM/Ev2, Ansible)", "PowerShell", "Kusto (KQL)",
                      "Geneva", "RBAC / IAM", "Entra ID", "Active Directory", "SCCM / MECM",
                      "PKI", "Docker", "Linux", "Incident Response"]
                        .into_iter()
                        .map(|s| view! { <li>{s}</li> })
                        .collect_view()}
                </ul>
            </Section>

            <Section title="Education">
                <ExperienceItem
                    role="Database Analysis Certificate"
                    company="Bellevue College"
                    dates="2016"
                    bullets=vec![]
                />
            </Section>

            <footer class="footer">
                <p>"Built with Rust + Leptos (WASM). \u{00A9} 2026 Mario J. Rivas."</p>
            </footer>
        </main>
    }
}
