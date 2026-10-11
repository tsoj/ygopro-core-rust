//! Demiurge Ema
use super::prelude::*;

pub const ID: u32 = 26364381;
const TOKEN_HOMUNCULUS: u32 = ID + 1;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[TOKEN_HOMUNCULUS], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // You can send 4 monsters with 2400 or more ATK and 1000 DEF from your hand, Deck, and/or face-up field to the GY; Special Summon this card from your hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCost(s.selfspcost)
    effect::set_cost(d, e1, Some(cost_target(selfspcost)));
    // e1:SetTarget(s.selfsptg)
    effect::set_target(d, e1, Some(cost_target(selfsptg)));
    // e1:SetOperation(s.selfspop)
    effect::set_operation(d, e1, Some(operation(selfspop)));
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // You can target 1 Spell/Trap on each field; destroy them, then Special Summon 1 "Homunculus Token" (Fairy/LIGHT/Level 2/ATK 800/DEF 800) to both players' fields in Defense Position, and if you do, this card gains 1600 ATK
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_DESTROY+CATEGORY_TOKEN+CATEGORY_SPECIAL_SUMMON+CATEGORY_ATKCHANGE)
    effect::set_category(d, e2, CATEGORY_DESTROY + CATEGORY_TOKEN + CATEGORY_SPECIAL_SUMMON + CATEGORY_ATKCHANGE);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetTarget(s.tkntg)
    effect::set_target(d, e2, Some(cost_target(tkntg)));
    // e2:SetOperation(s.tknop)
    effect::set_operation(d, e2, Some(operation(tknop)));
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.selfspcostfilter(c)
fn selfspcostfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAttackAbove(2400) and c:IsDefense(1000) and (c:IsFaceup() or not c:IsLocation(LOCATION_MZONE)) and c:IsAbleToGraveAsCost()
    card::is_attack_above(d, c, 2400)
        && card::is_defense(d, c, &[1000])
        && (card::is_faceup(d, c) || !card::is_location(d, c, LOCATION_MZONE))
        && card::is_able_to_grave_as_cost(d, c)
}
// function s.selfspcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn selfspcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(s.selfspcostfilter,tp,LOCATION_HAND|LOCATION_MZONE|LOCATION_DECK,0,e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let g = duel::get_matching_group(d, Some(&selfspcostfilter), tp, LOCATION_HAND | LOCATION_MZONE | LOCATION_DECK, 0, h.into());
    // if chk==0 then return #g>=4 and Duel.GetMZoneCount(tp,g)>0 end
    if a.chk == Some(0) {
        return g.len() >= 4 && duel::get_m_zone_count(d, tp, (&g).into(), None, None, None).0 > 0;
    }
    // local sg=aux.SelectUnselectGroup(g,e,tp,4,4,aux.ChkfMMZ(1),1,tp,HINTMSG_TOGRAVE)
    let sg = aux::select_unselect_group(d, &g, Some(a.e), tp, Some(4), Some(4), Some(&aux::chkf_mmz(1)), 1, tp, Some(HINTMSG_TOGRAVE as u64), None, None, false).unwrap_or_default();
    // Duel.SendtoGrave(sg,REASON_COST)
    duel::send_to_grave(d, sg, REASON_COST, None, None);
    true
}
// function s.selfsptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn selfsptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, a.tp, 0);
    true
}
// function s.selfspop(e,tp,eg,ep,ev,re,r,rp)
fn selfspop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
// function s.cansummontokentothisplayer(sum_player,target_player)
fn cansummontokentothisplayer(d: &mut Duel, sum_player: u8, target_player: u8) -> bool {
    // return Duel.IsPlayerCanSpecialSummonMonster(sum_player,TOKEN_HOMUNCULUS,0,TYPES_TOKEN,800,800,2,RACE_FAIRY,ATTRIBUTE_LIGHT,POS_FACEUP_DEFENSE,target_player)
    duel::is_player_can_special_summon_monster(
        d,
        sum_player,
        TOKEN_HOMUNCULUS,
        duel::MonsterData {
            setcodes: Some(vec![0]),
            type_: Some(TYPES_TOKEN),
            attack: Some(800),
            defense: Some(800),
            level: Some(2),
            race: Some(RACE_FAIRY),
            attribute: Some(ATTRIBUTE_LIGHT),
        },
        Some(POS_FACEUP_DEFENSE as u8),
        Some(target_player),
        0,
    )
}
// function s.rescon(sg,e,tp,mg)
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:GetClassCount(Card.GetControler,nil)==2 and Duel.GetMZoneCount(tp,sg)>0 and Duel.GetMZoneCount(1-tp,sg)>0
    let classes = group::get_class_count(d, sg, &|d, c| card::get_controler(d, c) as i64);
    let r = classes == 2
        && duel::get_m_zone_count(d, tp, sg.into(), None, None, None).0 > 0
        && duel::get_m_zone_count(d, 1 - tp, sg.into(), None, None, None).0 > 0;
    (r, false)
}
// function s.tkntg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn tkntg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // local g=Duel.GetTargetGroup(Card.IsSpellTrap,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,nil)
    let g = duel::get_target_group(d, Some(&|d, c| card::is_spell_trap(d, c)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX);
    // if chk==0 then return #g>=2 and not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT)
    //     and s.cansummontokentothisplayer(tp,tp)
    //     and s.cansummontokentothisplayer(tp,1-tp)
    //     and aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,0)
    // end
    if a.chk == Some(0) {
        return g.len() >= 2
            && duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_none()
            && cansummontokentothisplayer(d, tp, tp)
            && cansummontokentothisplayer(d, tp, 1 - tp)
            && matches!(
                aux::select_unselect_group(d, &g, Some(a.e), tp, Some(2), Some(2), Some(&rescon), 0, tp, None, None, None, false),
                Err(true)
            );
    }
    // local tg=aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,1,tp,HINTMSG_DESTROY)
    let tg = aux::select_unselect_group(d, &g, Some(a.e), tp, Some(2), Some(2), Some(&rescon), 1, tp, Some(HINTMSG_DESTROY as u64), None, None, false).unwrap_or_default();
    // Duel.SetTargetCard(tg)
    duel::set_target_card(d, tg.clone());
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,tg,#tg,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(tg.clone()), tg.len() as u8, tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,0,2,PLAYER_ALL,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 2, PLAYER_ALL as u8, 0);
    // Duel.SetOperationInfo(0,CATEGORY_TOKEN,0,2,PLAYER_ALL,0)
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, 2, PLAYER_ALL as u8, 0);
    // Duel.SetOperationInfo(0,CATEGORY_ATKCHANGE,e:GetHandler(),1,tp,1600)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_ATKCHANGE, Some(CardSet::from_card(h)), 1, tp, 1600);
    true
}
// function s.tknop(e,tp,eg,ep,ev,re,r,rp)
fn tknop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tg=Duel.GetTargetCards(e)
    let tg = duel::get_target_cards(d, a.e).nil();
    // if #tg>0 and Duel.Destroy(tg,REASON_EFFECT)>0 and not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT)
    //     and Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and Duel.GetLocationCount(1-tp,LOCATION_MZONE)>0
    //     and s.cansummontokentothisplayer(tp,tp) and s.cansummontokentothisplayer(tp,1-tp) then
    if !tg.is_empty()
        && duel::destroy(d, tg, REASON_EFFECT, None, None) > 0
        && duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_none()
        && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::get_location_count(d, 1 - tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && cansummontokentothisplayer(d, tp, tp)
        && cansummontokentothisplayer(d, tp, 1 - tp)
    {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // local token1=Duel.CreateToken(tp,TOKEN_HOMUNCULUS)
        let token1 = duel::create_token(d, tp, TOKEN_HOMUNCULUS).nil();
        // Duel.SpecialSummonStep(token1,0,tp,tp,false,false,POS_FACEUP_DEFENSE)
        duel::special_summon_step(d, token1, 0, tp, tp, false, false, POS_FACEUP_DEFENSE as u8, None);
        // local token2=Duel.CreateToken(tp,TOKEN_HOMUNCULUS)
        let token2 = duel::create_token(d, tp, TOKEN_HOMUNCULUS).nil();
        // Duel.SpecialSummonStep(token2,0,tp,1-tp,false,false,POS_FACEUP_DEFENSE)
        duel::special_summon_step(d, token2, 0, tp, 1 - tp, false, false, POS_FACEUP_DEFENSE as u8, None);
        // if Duel.SpecialSummonComplete()==0 then return end
        if duel::special_summon_complete(d) == 0 {
            return;
        }
        // local c=e:GetHandler()
        let c = effect::get_handler(d, a.e).nil();
        // if c:IsRelateToEffect(e) and c:IsFaceup() then
        if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) {
            // --This card gains 1600 ATK
            // c:UpdateAttack(1600)
            card::update_attack(d, c, 1600, None, None, None);
        }
    }
}
