//! Chimeratech Fortress Dragon
use super::prelude::*;

pub const ID: u32 = 79229522;

pub const INFO: ScriptInfo = ScriptInfo { material_setcode: &[SET_CYBER, SET_CYBER_DRAGON], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // fusion material
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMixRep(c,true,true,s.fil,1,99,CARD_CYBER_DRAGON)
    fusion::add_proc_mix_rep(
        d,
        c,
        true,
        true,
        fusion::Mat::F(fusion::fus_mat_full(|d, c, a| fil(d, c, a))),
        1,
        99,
        vec![fusion::Mat::Code(CARD_CYBER_DRAGON)],
    );
    // Fusion.AddContactProc(c,s.contactfil,s.contactop,s.splimit)
    fusion::add_contact_proc(
        d,
        c,
        fusion::Contact {
            material_filter: Some(std::sync::Arc::new(contactfil)),
            material_operation: Some(std::sync::Arc::new(contactop)),
            sumcon: Some(Some(lua_fn(splimit))),
            ..Default::default()
        },
    );
    // cannot be fusion material
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e3:SetCode(EFFECT_CANNOT_BE_FUSION_MATERIAL)
    effect::set_code(d, e3, EFFECT_CANNOT_BE_FUSION_MATERIAL);
    // e3:SetValue(1)
    effect::set_value(d, e3, 1);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.fil(c,fc,sumtype,tp,sub,mg,sg,contact)`
fn fil(d: &mut Duel, c: CardId, a: &fusion::MatArgs) -> bool {
    // if contact then sumtype=0 end
    let sumtype = if a.contact { 0 } else { a.sumtype };
    // return c:IsRace(RACE_MACHINE,fc,sumtype,tp) and (not contact or c:IsType(TYPE_MONSTER,fc,sumtype,tp))
    card::is_race_ctx(d, c, RACE_MACHINE, Some(a.fc), sumtype, Some(a.tp)) && (!a.contact || card::is_type_ctx(d, c, TYPE_MONSTER, Some(a.fc), sumtype, Some(a.tp)))
}
/// `s.splimit(e,se,sp,st)`
fn splimit(d: &mut Duel, a: Args) -> bool {
    // return e:GetHandler():GetLocation()~=LOCATION_EXTRA
    let h = effect::get_handler(d, a.e(0)).nil();
    card::get_location(d, h) != LOCATION_EXTRA
}
/// `s.cfilter(c,tp)`
fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsAbleToGraveAsCost() and (c:IsControler(tp) or c:IsFaceup())
    card::is_able_to_grave_as_cost(d, c) && (card::is_controler(d, c, tp) || card::is_faceup(d, c))
}
/// `s.contactfil(tp)`
fn contactfil(d: &mut Duel, tp: u8) -> CardSet {
    // return Duel.GetMatchingGroup(s.cfilter,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,nil,tp)
    duel::get_matching_group(d, Some(&|d, c| cfilter(d, c, tp)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX)
}
/// `s.contactop(g,tp,c)`
fn contactop(d: &mut Duel, g: &CardSet, _tp: u8, c: CardId) {
    // Duel.SendtoGrave(g,REASON_COST|REASON_MATERIAL)
    duel::send_to_grave(d, g.clone(), REASON_COST | REASON_MATERIAL, None, None);
    // spsummon condition
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_SET_BASE_ATTACK)
    effect::set_code(d, e1, EFFECT_SET_BASE_ATTACK);
    // e1:SetReset(RESET_EVENT|(RESETS_STANDARD_DISABLE&~RESET_TOFIELD))
    effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD_DISABLE & !RESET_TOFIELD), 1);
    // e1:SetValue(#g*1000)
    effect::set_value(d, e1, g.len() as i64 * 1000);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
