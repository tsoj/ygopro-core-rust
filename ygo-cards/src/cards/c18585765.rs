//! Voltester
//! ボルテスター
use super::prelude::*;

pub const ID: u32 = 18585765;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // destroy
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e1:SetProperty(EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e1, EFFECT_FLAG_DAMAGE_STEP, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.destg)
    effect::set_target(d, e1, Some(cost_target(destg)));
    // e1:SetOperation(s.desop)
    effect::set_operation(d, e1, Some(operation(desop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.lkfilter(c,oc)`
fn lkfilter(d: &mut Duel, c: CardId, oc: CardId) -> bool {
    // return c:GetLinkedGroup():IsContains(oc)
    let lg = card::get_linked_group(d, c);
    group::is_contains(&lg, oc)
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.IsExistingMatchingCard(s.lkfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil,c) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, x| lkfilter(d, x, c)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    // local g=Duel.GetMatchingGroup(s.lkfilter,tp,LOCATION_MZONE,LOCATION_MZONE,nil,c)
    let g = duel::get_matching_group(d, Some(&|d, x| lkfilter(d, x, c)), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, 0, 0);
    true
}
/// `s.GetPointedGroup(g)`
fn get_pointed_group(d: &mut Duel, g: &CardSet) -> CardSet {
    // local og=Group.CreateGroup()
    let mut og = CardSet::new();
    // for tc in aux.Next(g) do
    for tc in group::iter(d, g) {
        // local lg=tc:GetLinkedGroup()
        let lg = card::get_linked_group(d, tc);
        // for lc in aux.Next(lg) do
        for lc in group::iter(d, &lg) {
            // lc:RegisterFlagEffect(tc:GetFieldID(),RESET_CHAIN,0,0)
            let fid = card::get_field_id(d, tc);
            card::register_flag_effect(d, lc, fid, RESET_CHAIN, 0, 0, 0, 0);
            // og:AddCard(lc)
            group::add_card(&mut og, &CardSet::from_card(lc));
        }
    }
    // return og
    og
}
/// `s.idfilter(c,og)`
fn idfilter(d: &mut Duel, c: CardId, og: &CardSet) -> bool {
    // for tc in aux.Next(og) do
    for tc in group::iter(d, og) {
        // if c:GetFlagEffect(tc:GetFieldID()) then return true end
        // (GetFlagEffect returns a number, and a Lua number is always true, even 0)
        let fid = card::get_field_id(d, tc);
        let _ = card::get_flag_effect(d, c, fid);
        return true;
    }
    // return false
    false
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local g=Duel.GetMatchingGroup(s.lkfilter,tp,LOCATION_MZONE,LOCATION_MZONE,nil,c)
    let mut g = duel::get_matching_group(d, Some(&|d, x| lkfilter(d, x, c)), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // if #g>0 then
    if !g.is_empty() {
        // local lg=s.GetPointedGroup(g)
        let mut lg = get_pointed_group(d, &g);
        // if Duel.Destroy(g,REASON_EFFECT)~=0 then
        if duel::destroy(d, g.clone(), REASON_EFFECT, None, None) != 0 {
            // local og=Duel.GetOperatedGroup()
            let og = duel::get_operated_group(d);
            // g=lg:Filter(s.idfilter,c,og)
            g = group::filter(d, &lg, &|d, x| idfilter(d, x, &og), c.into());
            // while #g>0 do
            while !g.is_empty() {
                // Duel.BreakEffect()
                duel::break_effect(d);
                // lg=s.GetPointedGroup(g)
                lg = get_pointed_group(d, &g);
                // Duel.Destroy(g,REASON_EFFECT)
                duel::destroy(d, g.clone(), REASON_EFFECT, None, None);
                // local og=Duel.GetOperatedGroup()
                let og = duel::get_operated_group(d);
                // g=lg:Filter(s.idfilter,c,og)
                g = group::filter(d, &lg, &|d, x| idfilter(d, x, &og), c.into());
            }
        }
    }
}
