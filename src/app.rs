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
                    "Cloud Support Engineer and Site Reliability practitioner with 7+ years "
                    "keeping Azure government-cloud infrastructure at 99.99% uptime. "
                    "Background spanning IAM microservices, CI/CD automation, and Navy "
                    "intelligence analysis. Based in the Pacific Northwest."
                </p>
            </Section>

            <Section title="Experience">
                <ExperienceItem
                    role="Cloud Support Engineer 2"
                    company="ASM Research (Microsoft Vendor)"
                    dates="Dec 2018 \u{2014} Present"
                    bullets=vec![
                        "Ensured 99.99% uptime of Azure cloud infrastructure across 4+ government clouds.",
                        "Contributed 100+ PRs to the MSODS org repo (scripts, deploy configs, ARM Ev2) through code review, merge, and test pipelines.",
                        "Automated deployments with Azure DevOps CI/CD and IaC; built VSRM pipelines to ship patches, microservice versions, and tool configs to Gov Clouds in ring deployments.",
                        "Managed Azure IAM microservices: Service Fabric replicas (baremetal), AKS, and Cosmos instances at 99.99% uptime.",
                        "Built logging/monitoring with Kusto and Geneva to proactively catch and prevent service outages.",
                        "Configured WDS and SCCM/MECM for Windows Server deployment, patching, and imaging across hundreds of servers.",
                    ]
                />
                <ExperienceItem
                    role="Service Engineer"
                    company="Avanade (Microsoft Vendor)"
                    dates="Aug 2017 \u{2014} Dec 2018"
                    bullets=vec![
                        "Deployed Kubernetes v1.21 clusters on Linux via Azure IaaS, cutting infra cost 25%; Ansible v2.10 added 15% management savings.",
                        "Led DevSecOps integration with an IaC toolchain (key rotations, config management), improving reliability/uptime 20%.",
                        "Applied Azure Key Vault, NSG rules, RBAC, and IAM to address 250+ client security challenges annually.",
                        "Built a custom CI/CD pipeline for file management, Docker builds, and Container Registry updates, cutting manual work 30%.",
                        "Engineered resilient cloud networks (VMs, VMSS, VNETs, load balancers) with auto-scaling and internal file sharing.",
                    ]
                />
                <ExperienceItem
                    role="Intelligence Specialist / Expeditionary Warfare Analyst"
                    company="US Navy / US Navy Reserves"
                    dates="2018 \u{2014} 2024"
                    bullets=vec![
                        "Served in the Pacific at Combined Naval Forces Korea and Naval Special Warfare Group 2 as maritime, special forces, and counterintelligence analyst.",
                        "Analyzed drone Full Motion Video (FMV) and Geospatial Imagery (GEOINT) using Google Earth and ArcGIS.",
                        "Delivered daily Red Force and maritime briefs to senior leadership and pre-mission briefs to Special Boat Team commanders.",
                    ]
                />
                <ExperienceItem
                    role="System Administrator"
                    company="Leidos (Microsoft Vendor)"
                    dates="Feb 2017 \u{2014} Aug 2017"
                    bullets=vec![
                        "Managed network infrastructure for 1000 users (Domain Controllers, Exchange, Active Directory), driving 98% user satisfaction.",
                        "Handled 120+ tickets monthly; gained PKI exposure managing digital certificates.",
                    ]
                />
                <ExperienceItem
                    role="Service Engineer Intern"
                    company="Microsoft"
                    dates="Aug 2016 \u{2014} Feb 2017"
                    bullets=vec![
                        "Ran a compliance-tool assessment for the Windows Devices group, uncovering 50% potential savings and advocating an external vendor solution.",
                    ]
                />
            </Section>

            <Section title="Projects">
                <ExperienceItem
                    role="Secure Proxmox Homelab"
                    company="Personal"
                    dates=""
                    bullets=vec![
                        "Built a Proxmox homelab with Azure Entra ID integration via an OAuth2 proxy on Ubuntu for centralized auth and access control.",
                    ]
                />
                <ExperienceItem
                    role="PowerShell Automation Tooling"
                    company="Personal"
                    dates=""
                    bullets=vec![
                        "Built two PowerShell tools with UIs: cut replica server OS migration from 6h to 4h (33%) and reduced user error 35%.",
                        "Wrote a cloud-agnostic script to scan baremetal servers, chassis managers, and VM inventories for accurate service deconfliction.",
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
                    {["Azure", "Kubernetes / AKS", "Service Fabric", "CI/CD (Azure DevOps)",
                      "Infrastructure as Code", "PowerShell", "Kusto", "Geneva",
                      "RBAC / IAM", "SCCM / MECM", "Linux", "Docker"]
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
