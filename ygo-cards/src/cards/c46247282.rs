//! Iron Dragon Tiamaton
use super::prelude::*;

pub const ID: u32 = 46247282;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // --cannot special summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // --special summon
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_HAND)
    effect::set_range(d, e2, LOCATION_HAND);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(s.spcon)
    effect::set_condition(d, e2, Some(condition(spcon)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // --destroy
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e3, CATEGORY_DESTROY);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e3:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    // e3:SetTarget(s.destg)
    effect::set_target(d, e3, Some(cost_target(destg)));
    // e3:SetOperation(s.desop)
    effect::set_operation(d, e3, Some(operation(desop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // --disable field
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetCode(EFFECT_DISABLE_FIELD)
    effect::set_code(d, e4, EFFECT_DISABLE_FIELD);
    // e4:SetProperty(EFFECT_FLAG_REPEAT)
    effect::set_property(d, e4, EFFECT_FLAG_REPEAT, 0);
    // e4:SetOperation(s.disop)
    effect::set_operation(d, e4, Some(lua_fn(disop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:GetColumnGroupCount()>1
    card::get_column_group_count(d, c, 0, 0) > 1
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil)
    duel::is_existing_matching_card(d, Some(&cfilter), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and e:GetHandler():IsCanBeSpecialSummoned(e,0,tp,true,true) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, h, a.e, 0, a.tp, true, true, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,e:GetHandler(),1,0,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and Duel.SpecialSummon(c,0,tp,tp,true,true,POS_FACEUP)~=0 then
    if card::is_relate_to_effect(d, c, a.e) && duel::special_summon(d, CardSet::from_card(c), 0, a.tp, a.tp, true, true, POS_FACEUP as u8, None) != 0 {
        // c:CompleteProcedure()
        card::complete_procedure(d, c);
    }
}
/// `s.desfilter(c,g)`
fn desfilter(c: CardId, g: &CardSet) -> bool {
    // return g:IsContains(c)
    group::is_contains(g, c)
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local cg=e:GetHandler():GetColumnGroup()
    let h = effect::get_handler(d, a.e).nil();
    let cg = card::get_column_group(d, h, 0, 0);
    // local g=Duel.GetMatchingGroup(s.desfilter,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,e:GetHandler(),cg)
    let g = duel::get_matching_group(d, Some(&|_d, c| desfilter(c, &cg)), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, h.into());
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, 0, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local cg=c:GetColumnGroup()
    let cg = card::get_column_group(d, c, 0, 0);
    // if c:IsRelateToEffect(e) and c:IsFaceup() then
    if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) {
        // local g=Duel.GetMatchingGroup(s.desfilter,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,c,cg)
        let g = duel::get_matching_group(d, Some(&|_d, x| desfilter(x, &cg)), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, c.into());
        // if #g>0 then
        if !g.is_empty() {
            // Duel.Destroy(g,REASON_EFFECT)
            duel::destroy(d, g, REASON_EFFECT, None, None);
        }
    }
}
/// `s.disop(e,tp)`: the zones the columns of this card block
fn disop(d: &mut Duel, a: Args) -> i64 {
    // local c=e:GetHandler()
    let h = effect::get_handler(d, a.e(0)).nil();
    let tp = a.p(1);
    // local zone=c:GetColumnZone(LOCATION_ONFIELD)
    let mut zone = card::get_column_zone(d, h, LOCATION_ONFIELD, 0, 0, None) as i64;
    // for tc in e:GetHandler():GetColumnGroup():Iter() do
    let cg = card::get_column_group(d, h, 0, 0);
    for tc in group::iter(d, &cg) {
        // local dz=tc:IsLocation(LOCATION_MZONE) and 1 or (1 << 8)
        let mut dz: i64 = if card::is_location(d, tc, LOCATION_MZONE) { 1 } else { 1 << 8 };
        if card::is_sequence(d, tc, &[5, 6]) {
            let seq = card::get_sequence(d, tc);
            // local dz1=tc:IsControler(tp) and (dz << tc:GetSequence()) or (dz << (16 + tc:GetSequence()))
            let dz1 = if card::is_controler(d, tc, tp) { dz << seq } else { dz << (16 + seq) };
            // local dz2=tc:IsControler(tp) and (dz << (16 + (11 - tc:GetSequence()))) or (dz << (11 - tc:GetSequence()))
            let dz2 = if card::is_controler(d, tc, tp) { dz << (16 + (11 - seq)) } else { dz << (11 - seq) };
            // dz=dz1|dz2
            dz = dz1 | dz2;
        } else {
            let seq = card::get_sequence(d, tc);
            // dz=tc:IsControler(tp) and (dz << tc:GetSequence()) or (dz << (16 + tc:GetSequence()))
            dz = if card::is_controler(d, tc, tp) { dz << seq } else { dz << (16 + seq) };
        }
        // zone=zone&~dz
        zone &= !dz;
    }
    // return zone
    zone
}
