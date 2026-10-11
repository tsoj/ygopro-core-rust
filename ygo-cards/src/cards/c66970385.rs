//! Last Chapter of the Noble Knights
use super::prelude::*;

pub const ID: u32 = 66970385;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_NOBLE_ARMS, SET_NOBLE_KNIGHT], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // If your opponent controls a monster and you control no monsters: You can target 1 "Noble Arms" Equip Spell Card and 1 appropriate "Noble Knight" monster in your Graveyard; ...
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_EQUIP)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_EQUIP);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetFieldGroupCount(tp,0,LOCATION_MZONE)>0 and Duel.GetFieldGroupCount(tp,LOCATION_MZONE,0)==0
    duel::get_field_group_count(d, a.tp, 0, LOCATION_MZONE) > 0 && duel::get_field_group_count(d, a.tp, LOCATION_MZONE, 0) == 0
}
/// `s.tgfilter(c,e,tp)`
fn tgfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return (c:IsSetCard(SET_NOBLE_ARMS) and c:IsEquipSpell() and c:CheckUniqueOnField(tp))
    //     or (c:IsSetCard(SET_NOBLE_KNIGHT) and c:IsMonster() and c:IsCanBeSpecialSummoned(e,0,tp,false,false))
    (card::is_set_card(d, c, &[SET_NOBLE_ARMS as u16])
        && card::is_equip_spell(d, c)
        && card::check_unique_on_field(d, c, tp, None, None))
        || (card::is_set_card(d, c, &[SET_NOBLE_KNIGHT as u16])
            && card::is_monster(d, c)
            && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None))
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // local monster=sg:Filter(Card.IsMonster,nil):GetFirst()
    let monsters = group::filter(d, sg, &|d, c| card::is_monster(d, c), NO_EX);
    let monster = group::get_first(d, &monsters);
    // local spell=sg:Filter(Card.IsEquipSpell,nil):GetFirst()
    let spells = group::filter(d, sg, &|d, c| card::is_equip_spell(d, c), NO_EX);
    let spell = group::get_first(d, &spells);
    // return monster and spell and spell:CheckEquipTarget(monster)
    let res = match (monster, spell) {
        (Some(m), Some(s)) => card::check_equip_target(d, s, m),
        _ => false,
    };
    (res, false)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // local g=Duel.GetTargetGroup(s.tgfilter,tp,LOCATION_GRAVE,0,nil,e,tp)
    let g = duel::get_target_group(d, Some(&|d, c| tgfilter(d, c, a.e, tp)), tp, LOCATION_GRAVE, 0, NO_EX);
    // if chk==0 then
    if a.chk == Some(0) {
        // local ft=Duel.GetLocationCount(tp,LOCATION_SZONE)
        let (mut ft, _) = duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None);
        // if e:IsHasType(EFFECT_TYPE_ACTIVATE) and e:GetHandler():IsLocation(LOCATION_HAND) then ft=ft-1 end
        if effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE) {
            let h = effect::get_handler(d, a.e).nil();
            if card::is_location(d, h, LOCATION_HAND) {
                ft -= 1;
            }
        }
        // return ft>0 and Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and #g>=2 and aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,0)
        if ft <= 0 {
            return false;
        }
        let (mct, _) = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None);
        if mct <= 0 || g.len() < 2 {
            return false;
        }
        return matches!(
            aux::select_unselect_group(d, &g, Some(a.e), tp, Some(2), Some(2), Some(&rescon), 0, tp, None, None, None, false),
            Err(true)
        );
    }
    // local tg=aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,1,tp,HINTMSG_TARGET)
    let tg = aux::select_unselect_group(d, &g, Some(a.e), tp, Some(2), Some(2), Some(&rescon), 1, tp, Some(HINTMSG_TARGET as u64), None, None, false).unwrap_or_default();
    // Duel.SetTargetCard(tg)
    duel::set_target_card(d, tg.clone());
    // local monster,spell=tg:Split(Card.IsMonster,nil)
    let (monster, spell) = group::split(d, &tg, &|d, c| card::is_monster(d, c), NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,monster,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(monster), 1, tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_EQUIP,spell,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, Some(spell), 1, tp, 0);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tg=Duel.GetTargetCards(e)
    // if #tg==0 then return end
    let tg = duel::get_target_cards(d, a.e).unwrap_or_default();
    if tg.is_empty() {
        return;
    }
    // local monster=tg:Filter(Card.IsMonster,nil):GetFirst()
    let monsters = group::filter(d, &tg, &|d, c| card::is_monster(d, c), NO_EX);
    let monster = group::get_first(d, &monsters);
    // local spell=tg:Filter(Card.IsEquipSpell,nil):GetFirst()
    let spells = group::filter(d, &tg, &|d, c| card::is_equip_spell(d, c), NO_EX);
    let spell = group::get_first(d, &spells);
    // if monster and Duel.SpecialSummon(monster,0,tp,tp,false,false,POS_FACEUP)>0 and spell
    //     and spell:CheckUniqueOnField(tp) and spell:CheckEquipTarget(monster) then
    if let Some(monster) = monster {
        if duel::special_summon(d, monster, 0, tp, tp, false, false, POS_FACEUP as u8, None) > 0 {
            if let Some(spell) = spell {
                if card::check_unique_on_field(d, spell, tp, None, None) && card::check_equip_target(d, spell, monster) {
                    // Duel.Equip(tp,spell,monster)
                    duel::equip(d, tp, spell, monster, true, false);
                }
            }
        }
    }
}
