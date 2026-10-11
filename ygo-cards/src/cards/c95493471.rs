//! Neo Super Quantal Mech King Blaster Magna
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 95493471;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_SUPER_QUANT],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsType,TYPE_EFFECT),2,nil,s.matcheck)
    link::add_procedure(d, c, Some(mat_filter(|d, c, sc, st, tp| card::is_type_ctx(d, c, TYPE_EFFECT, sc, st, Some(tp)))), 2, None, Some(group_check(matcheck)), None, None);
    // Prevent destruction by opponent's effect
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_EFFECT);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCondition(s.incon)
    effect::set_condition(d, e1, Some(condition(incon)));
    // e1:SetValue(aux.indoval)
    effect::set_value_fn(d, e1, aux::indoval());
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Draw 1 card
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_DRAW)
    effect::set_category(d, e2, CATEGORY_DRAW);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e2:SetCondition(s.drcon)
    effect::set_condition(d, e2, Some(condition(drcon)));
    // e2:SetTarget(s.drtg)
    effect::set_target(d, e2, Some(cost_target(drtg)));
    // e2:SetOperation(s.drop)
    effect::set_operation(d, e2, Some(operation(drop_op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Special Summon 1 "Super Quant" monster
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e3, EVENT_DESTROYED);
    // e3:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DELAY, 0);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1,id)
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    // e3:SetCondition(s.spcon)
    effect::set_condition(d, e3, Some(condition(spcon)));
    // e3:SetTarget(s.sptg)
    effect::set_target(d, e3, Some(cost_target(sptg)));
    // e3:SetOperation(s.spop)
    effect::set_operation(d, e3, Some(operation(spop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.matcheck(g,lc,sumtype,tp)`
fn matcheck(d: &mut Duel, g: &CardSet, lc: CardId, sumtype: u64, tp: u8) -> bool {
    // return g:IsExists(Card.IsSetCard,1,nil,SET_SUPER_QUANT,lc,sumtype,tp)
    group::is_exists(d, g, &|d, x| card::is_set_card_ctx(d, x, &[SET_SUPER_QUANT as u16], Some(lc), sumtype, Some(tp)), 1, NO_EX)
}
/// `s.incon(e)`
fn incon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsLinkSummoned()
    let h = effect::get_handler(d, a.e).nil();
    card::is_link_summoned(d, h)
}
/// `s.drcfilter(c,tp,lg)`
fn drcfilter(d: &mut Duel, c: CardId, tp: u8, lg: &CardSet) -> bool {
    // return c:IsSetCard(SET_SUPER_QUANT) and c:IsType(TYPE_XYZ) and c:IsPreviousLocation(LOCATION_EXTRA)
    //     and lg:IsContains(c) and not Duel.IsExistingMatchingCard(s.drfilter,tp,LOCATION_MZONE,0,1,c,c:GetCode())
    if !card::is_set_card(d, c, &[SET_SUPER_QUANT as u16]) || !card::is_type(d, c, TYPE_XYZ) || !card::is_previous_location(d, c, LOCATION_EXTRA) || !group::is_contains(lg, c) {
        return false;
    }
    let code = card::get_code(d, c)[0];
    !duel::is_existing_matching_card(d, Some(&|d, x| drfilter(d, x, code)), tp, LOCATION_MZONE, 0, 1, c.into())
}
/// `s.drfilter(c,code)`
fn drfilter(d: &mut Duel, c: CardId, code: u32) -> bool {
    card::is_faceup(d, c) && card::is_code(d, c, &[code])
}
/// `s.drcon(e,tp,eg,ep,ev,re,r,rp)`
fn drcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.drcfilter,1,nil,tp,e:GetHandler():GetLinkedGroup())
    let eg = a.eg.clone().nil();
    let h = effect::get_handler(d, a.e).nil();
    let lg = card::get_linked_group(d, h);
    group::is_exists(d, &eg, &|d, c| drcfilter(d, c, a.tp, &lg), 1, NO_EX)
}
/// `s.drtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn drtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, a.tp);
    // Duel.SetTargetParam(1)
    duel::set_target_param(d, 1);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 1);
    true
}
/// `s.drop(e,tp,eg,ep,ev,re,r,rp)`
fn drop_op(d: &mut Duel, _a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    // Duel.Draw(p,d,REASON_EFFECT)
    duel::draw(d, info[0].int() as u8, info[1].int() as u32, REASON_EFFECT);
}
/// `s.cfilter(c,tp,zone,rp)`
fn cfilter(d: &mut Duel, c: CardId, tp: u8, zone: u32, rp: Option<u8>) -> bool {
    // local seq=c:GetPreviousSequence()
    let mut seq = card::get_previous_sequence(d, c);
    // if not c:IsPreviousControler(tp) then seq=seq+16 end
    if !card::is_previous_controler(d, c, tp) {
        seq += 16;
    }
    // return ((c:IsReason(REASON_BATTLE)) or (c:IsReason(REASON_EFFECT) and rp~=tp)) and c:IsType(TYPE_XYZ)
    //     and c:IsPreviousLocation(LOCATION_MZONE) and bit.extract(zone,seq)~=0
    let reason_ok = card::is_reason(d, c, REASON_BATTLE) || (card::is_reason(d, c, REASON_EFFECT) && rp != Some(tp));
    reason_ok
        && card::is_type(d, c, TYPE_XYZ)
        && card::is_previous_location(d, c, LOCATION_MZONE)
        && seq < 32
        && ((zone >> seq) & 1) != 0
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local zone=e:GetHandler():GetLinkedZone()
    let h = effect::get_handler(d, a.e).nil();
    let zone = card::get_linked_zone(d, h, None);
    // return eg:IsExists(s.cfilter,1,nil,tp,zone,rp)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| cfilter(d, c, a.tp, zone, Some(a.rp)), 1, NO_EX)
}
/// `s.spfilter(c,e,tp,attr)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, attr: i64) -> bool {
    // return c:IsSetCard(SET_SUPER_QUANT) and c:GetOriginalAttribute()&attr~=0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_SUPER_QUANT as u16])
        && (card::get_original_attribute(d, c) as i64 & attr) != 0
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=eg:Filter(s.cfilter,nil,tp,e:GetHandler():GetLinkedZone())
    let h = effect::get_handler(d, a.e).nil();
    let zone = card::get_linked_zone(d, h, None);
    let eg = a.eg.clone().nil();
    let g = group::filter(d, &eg, &|d, c| cfilter(d, c, a.tp, zone, None), NO_EX);
    // local attr=0
    let mut attr: i64 = 0;
    // for tc in aux.Next(g) do
    for tc in group::iter(d, &g) {
        // attr=attr|tc:GetOriginalAttribute()
        attr |= card::get_original_attribute(d, tc) as i64;
    }
    // if chk==0 then
    if a.chk == Some(0) {
        // return attr~=0 and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
        //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_DECK,0,1,nil,e,tp,attr)
        return attr != 0
            && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp, attr)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // e:SetLabel(attr)
    effect::set_label(d, a.e, &[attr]);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<1 then return end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 < 1 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_DECK,0,1,1,nil,e,tp,e:GetLabel())
    let attr = effect::get_label(d, a.e);
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| spfilter(d, c, a.e, a.tp, attr)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
