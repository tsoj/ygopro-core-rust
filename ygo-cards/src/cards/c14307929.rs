//! Diabell, Queen of the White Forest
//! Scripted by Hatter
use super::prelude::*;

pub const ID: u32 = 14307929;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // 1 Tuner + 1+ non-Tuner monsters
    // Synchro.AddProcedure(c,nil,1,1,Synchro.NonTuner(nil),1,99)
    synchro::add_procedure(d, c, None, 1, 1, Some(synchro::non_tuner(None)), 1, 99, None, None, None, None, None);
    // Add 1 Spell/Trap from the GY to the hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e1, CATEGORY_TOHAND);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function(e) return e:GetHandler():IsSynchroSummoned() and e:GetLabel()==1 end)
    effect::set_condition(d, e1, Some(condition(cond1)));
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Check Synchro Summon material
    // local e1a=Effect.CreateEffect(c)
    let e1a = effect::create_effect(d, c);
    // e1a:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1a, EFFECT_TYPE_SINGLE);
    // e1a:SetCode(EFFECT_MATERIAL_CHECK)
    effect::set_code(d, e1a, EFFECT_MATERIAL_CHECK);
    // e1a:SetValue(s.matcheck)
    effect::set_value_fn(d, e1a, effect_card_value(matcheck));
    // e1a:SetLabelObject(e1)
    effect::set_label_object(d, e1a, Value::Effect(e1));
    // c:RegisterEffect(e1a)
    card::register_effect(d, c, e1a, false);
    // Special Summon 1 Level 7 or lower Tuner Synchro Monster
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_CHAINING)
    effect::set_code(d, e2, EVENT_CHAINING);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(function(e,tp,eg,ep) return ep==1-tp end)
    effect::set_condition(d, e2, Some(condition(cond2)));
    // e2:SetCost(s.spcost)
    effect::set_cost(d, e2, Some(cost_target(spcost)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `function(e) return e:GetHandler():IsSynchroSummoned() and e:GetLabel()==1 end`
fn cond1(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::is_synchro_summoned(d, c) && effect::get_label(d, a.e) == 1
}
/// `function(e,tp,eg,ep) return ep==1-tp end`
fn cond2(_d: &mut Duel, a: &EffArgs) -> bool {
    a.ep == 1 - a.tp
}
/// `s.matfilter(c,sc,tp)`
fn matfilter(d: &mut Duel, c: CardId, sc: CardId, tp: u8) -> bool {
    // return c:IsType(TYPE_SYNCHRO,sc,SUMMON_TYPE_SYNCHRO|MATERIAL_SYNCHRO,tp) and c:IsType(TYPE_TUNER,sc,SUMMON_TYPE_SYNCHRO|MATERIAL_SYNCHRO,tp)
    let st = SUMMON_TYPE_SYNCHRO as u64 | MATERIAL_SYNCHRO as u64;
    card::is_type_ctx(d, c, TYPE_SYNCHRO, Some(sc), st, Some(tp)) && card::is_type_ctx(d, c, TYPE_TUNER, Some(sc), st, Some(tp))
}
/// `s.matcheck(e,c)`
fn matcheck(d: &mut Duel, e: EffectId, c: CardId) -> i64 {
    // local mg=c:GetMaterial()
    let mg = card::get_material(d, c);
    let lo = effect::get_label_object(d, e).as_effect().nil();
    // if #mg==0 then return e:GetLabelObject():SetLabel(0) end
    if mg.is_empty() {
        effect::set_label(d, lo, &[0]);
        return 0;
    }
    let hp = effect::get_handler_player(d, e);
    // if mg:IsExists(s.matfilter,1,nil,c,e:GetHandlerPlayer()) then
    if group::is_exists(d, &mg, &|d, m| matfilter(d, m, c, hp), 1, NO_EX) {
        // e:GetLabelObject():SetLabel(1)
        effect::set_label(d, lo, &[1]);
    } else {
        // e:GetLabelObject():SetLabel(0)
        effect::set_label(d, lo, &[0]);
    }
    0
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_GRAVE) and chkc:IsSpellTrap() and chkc:IsAbleToHand() end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, a.tp) && card::is_location(d, chkc, LOCATION_GRAVE) && card::is_spell_trap(d, chkc) && card::is_able_to_hand(d, chkc, None);
    }
    // if chk==0 then return Duel.IsExistingTarget(aux.AND(Card.IsSpellTrap,Card.IsAbleToHand),tp,LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&thfilter), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectTarget(tp,aux.AND(Card.IsSpellTrap,Card.IsAbleToHand),tp,LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&thfilter), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(g), 1, a.tp, 0);
    true
}
/// `aux.AND(Card.IsSpellTrap,Card.IsAbleToHand)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_spell_trap(d, c) && card::is_able_to_hand(d, c, None)
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.SendtoHand(tc,nil,REASON_EFFECT)
        duel::send_to_hand(d, CardSet::from_card(tc), None, REASON_EFFECT, None);
    }
}
/// `s.spfilter(c,e,tp,mc)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, mc: Option<CardId>) -> bool {
    // if not (c:IsType(TYPE_TUNER) and c:IsType(TYPE_SYNCHRO) and c:IsLevelBelow(7)
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false)) then return false end
    if !(card::is_type(d, c, TYPE_TUNER)
        && card::is_type(d, c, TYPE_SYNCHRO)
        && card::is_level_below(d, c, 7)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None))
    {
        return false;
    }
    // mc as the `ex` argument (nil = none)
    let ex: Exclude = match mc {
        Some(m) => m.into(),
        None => NO_EX,
    };
    // if c:IsLocation(LOCATION_EXTRA) then
    if card::is_location(d, c, LOCATION_EXTRA) {
        // return Duel.GetLocationCountFromEx(tp,tp,mc,c)>0
        duel::get_location_count_from_ex(d, tp, Some(tp), ex, duel::ExtraCard::Card(c), None).0 > 0
    } else {
        // return Duel.GetMZoneCount(tp,mc)>0
        duel::get_m_zone_count(d, tp, ex, None, None, None).0 > 0
    }
}
/// `s.spcostfilter(c,e,tp)`
fn spcostfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSpellTrap() and c:IsAbleToGraveAsCost()
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_EXTRA|LOCATION_GRAVE|LOCATION_REMOVED,0,1,nil,e,tp,c)
    card::is_spell_trap(d, c)
        && card::is_able_to_grave_as_cost(d, c)
        && duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, e, tp, Some(c))), tp, LOCATION_EXTRA | LOCATION_GRAVE | LOCATION_REMOVED, 0, 1, NO_EX)
}
/// `s.spcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.spcostfilter,tp,LOCATION_HAND|LOCATION_ONFIELD,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| spcostfilter(d, c, a.e, a.tp)), a.tp, LOCATION_HAND | LOCATION_ONFIELD, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spcostfilter,tp,LOCATION_HAND|LOCATION_ONFIELD,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| spcostfilter(d, c, a.e, a.tp)), a.tp, LOCATION_HAND | LOCATION_ONFIELD, 0, 1, 1, false, NO_EX).nil();
    // Duel.SendtoGrave(g,REASON_COST)
    duel::send_to_grave(d, g, REASON_COST, None, None);
    true
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA|LOCATION_GRAVE|LOCATION_REMOVED)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_EXTRA | LOCATION_GRAVE | LOCATION_REMOVED) as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.spfilter),tp,LOCATION_EXTRA|LOCATION_GRAVE|LOCATION_REMOVED,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(
        d,
        a.tp,
        Some(&aux::necro_valley_filter(|d, c| spfilter(d, c, a.e, a.tp, None))),
        a.tp,
        LOCATION_EXTRA | LOCATION_GRAVE | LOCATION_REMOVED,
        0,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
