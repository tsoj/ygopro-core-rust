//! Salamandra Fusion
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 9102835;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_FLAME_SWORDSMAN], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Equip only to a FIRE Warrior monster
    // aux.AddEquipProcedure(c,nil,s.eqfilter)
    aux::add_equip_procedure(d, c, None, Some(aux::equip_filter_fn(|d, c, _e, _tp| eqfilter(d, c))), None, None, None, None, None, None);
    // Equipped monster gains 700 ATK
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e1, EFFECT_TYPE_EQUIP);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetValue(700)
    effect::set_value(d, e1, 700);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Destruction replacement for the equipped monster
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_EQUIP+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_EQUIP + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EFFECT_DESTROY_REPLACE)
    effect::set_code(d, e2, EFFECT_DESTROY_REPLACE);
    // e2:SetTarget(s.replacetg)
    effect::set_target(d, e2, Some(cost_target(replacetg)));
    // e2:SetOperation(s.replaceop)
    effect::set_operation(d, e2, Some(operation(replaceop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Special Summon 1 "Flame Swordsman" or 1 Fusion Monster that mentions it from your Extra Deck
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_TOGRAVE+CATEGORY_SPECIAL_SUMMON+CATEGORY_FUSION_SUMMON)
    effect::set_category(d, e3, CATEGORY_TOGRAVE + CATEGORY_SPECIAL_SUMMON + CATEGORY_FUSION_SUMMON);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetCountLimit(1,id)
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    // e3:SetCondition(s.spcond)
    effect::set_condition(d, e3, Some(condition(spcond)));
    // e3:SetTarget(s.sptg)
    effect::set_target(d, e3, Some(cost_target(sptg)));
    // e3:SetOperation(s.spop)
    effect::set_operation(d, e3, Some(operation(spop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.eqfilter(c)`
fn eqfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAttribute(ATTRIBUTE_FIRE) and c:IsRace(RACE_WARRIOR)
    card::is_attribute(d, c, ATTRIBUTE_FIRE) && card::is_race(d, c, RACE_WARRIOR)
}
/// `s.replacetg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn replacetg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local ec=c:GetEquipTarget()
    let ec = card::get_equip_target(d, c);
    // if chk==0 then return ec:IsReason(REASON_EFFECT|REASON_BATTLE) and not ec:IsReason(REASON_REPLACE)
    //     and c:IsDestructable(e) and not c:IsStatus(STATUS_DESTROY_CONFIRMED) end
    if a.chk == Some(0) {
        let ec = ec.nil();
        return card::is_reason(d, ec, REASON_EFFECT | REASON_BATTLE)
            && !card::is_reason(d, ec, REASON_REPLACE)
            && card::is_destructable(d, c, Some(e))
            && !card::is_status(d, c, STATUS_DESTROY_CONFIRMED);
    }
    // if Duel.SelectEffectYesNo(tp,c,96) then
    if duel::select_effect_yes_no(d, tp, c, Some(96)) {
        // c:SetStatus(STATUS_DESTROY_CONFIRMED,true)
        card::set_status(d, c, STATUS_DESTROY_CONFIRMED, true);
        // return true
        true
    } else {
        // else return false end
        false
    }
}
/// `s.replaceop(e,tp,eg,ep,ev,re,r,rp,chk)`
fn replaceop(d: &mut Duel, a: &EffArgs) {
    // e:GetHandler():SetStatus(STATUS_DESTROY_CONFIRMED,false)
    let c = effect::get_handler(d, a.e).nil();
    card::set_status(d, c, STATUS_DESTROY_CONFIRMED, false);
    // Duel.Destroy(e:GetHandler(),REASON_EFFECT|REASON_REPLACE)
    duel::destroy(d, CardSet::from_card(c), REASON_EFFECT | REASON_REPLACE, None, None);
}
/// `s.spcond(e,tp,eg,ep,ev,re,r,rp)`
fn spcond(d: &mut Duel, a: &EffArgs) -> bool {
    // local ec=e:GetHandler():GetEquipTarget()
    let h = effect::get_handler(d, a.e).nil();
    let Some(ec) = card::get_equip_target(d, h) else {
        return false;
    };
    // return ec and ec:IsType(TYPE_FUSION) and ec:IsControler(tp)
    card::is_type(d, ec, TYPE_FUSION) && card::is_controler(d, ec, a.tp)
}
/// `s.spfilter(c,e,tp,mc)`: `mc` is the fourth argument (nil when the filter gets none)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, mc: Option<CardId>) -> bool {
    // return (c:IsCode(CARD_FLAME_SWORDSMAN) or (c:IsType(TYPE_FUSION) and c:ListsCode(CARD_FLAME_SWORDSMAN)))
    //     and Duel.GetLocationCountFromEx(tp,tp,mc,c)
    //     and c:IsCanBeSpecialSummoned(e,SUMMON_TYPE_FUSION,tp,false,false) and c:CheckFusionMaterial()
    let name_ok = card::is_code(d, c, &[CARD_FLAME_SWORDSMAN]) || (card::is_type(d, c, TYPE_FUSION) && card::lists_code(d, c, &[CARD_FLAME_SWORDSMAN]));
    if !name_ok {
        return false;
    }
    // A Lua number is always true here, even when it is 0
    let ex = match mc {
        Some(m) => m.into(),
        None => NO_EX,
    };
    let _ = duel::get_location_count_from_ex(d, tp, Some(tp), ex, duel::ExtraCard::Card(c), None);
    card::is_can_be_special_summoned(d, c, e, SUMMON_TYPE_FUSION, tp, false, false, None, None, None) && card::check_fusion_material(d, c, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local ec=c:GetEquipTarget()
    let ec = card::get_equip_target(d, c);
    // if chk==0 then return c:IsAbleToGrave() and ec:IsAbleToGrave()
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp,ec) end
    if a.chk == Some(0) {
        let ec = ec.nil();
        return card::is_able_to_grave(d, c, None)
            && card::is_able_to_grave(d, ec, None)
            && duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, e, tp, Some(ec))), tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,Group.FromCards(c,ec),2,tp,0)
    let ec = ec.nil();
    let mut cards = CardSet::from_card(c);
    group::add_card(&mut cards, &CardSet::from_card(ec));
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, Some(cards), 2, tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local ec=c:GetEquipTarget()
    let ec = card::get_equip_target(d, c);
    // if not (c:IsRelateToEffect(e) and ec) then return end
    if !card::is_relate_to_effect(d, c, e) {
        return;
    }
    let Some(ec) = ec else {
        return;
    };
    // local g=Group.FromCards(c,ec)
    let mut g = CardSet::from_card(c);
    group::add_card(&mut g, &CardSet::from_card(ec));
    // if Duel.SendtoGrave(g,REASON_EFFECT)==0 or g:FilterCount(Card.IsLocation,nil,LOCATION_GRAVE)==0 then return end
    if duel::send_to_grave(d, g.clone(), REASON_EFFECT, None, None) == 0 {
        return;
    }
    if group::filter_count(d, &g, &|d, x| card::is_location(d, x, LOCATION_GRAVE), NO_EX) == 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp,nil):GetFirst()
    let sel = duel::select_matching_card(d, tp, Some(&|d, x| spfilter(d, x, e, tp, None)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    let Some(sc) = group::get_first(d, &sel) else {
        return;
    };
    // sc:SetMaterial(nil)
    card::set_material(d, sc, CardSet::new());
    // if Duel.SpecialSummon(sc,SUMMON_TYPE_FUSION,tp,tp,false,false,POS_FACEUP)>0 then
    if duel::special_summon(d, CardSet::from_card(sc), SUMMON_TYPE_FUSION, tp, tp, false, false, POS_FACEUP as u8, None) > 0 {
        // sc:CompleteProcedure()
        card::complete_procedure(d, sc);
    }
}
