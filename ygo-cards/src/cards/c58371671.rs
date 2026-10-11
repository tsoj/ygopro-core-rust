//! Crystal Aegis
use super::prelude::*;

pub const ID: u32 = 58371671;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[ID + 1],
    listed_series: &[SET_CRYSTAL_BEAST],
    ..ScriptInfo::DEFAULT
};

/// `s.getprops(c)`: the ATK, DEF, Level, Race and Attribute of the Token copy of `c`
fn getprops(d: &mut Duel, c: CardId) -> (i32, i32, u32, u64, u32) {
    // return math.max(0,c:GetTextAttack()),math.max(0,c:GetTextDefense()),
    //     c:GetOriginalLevel(),c:GetOriginalRace(),c:GetOriginalAttribute()
    (
        card::get_text_attack(d, c).max(0),
        card::get_text_defense(d, c).max(0),
        card::get_original_level(d, c),
        card::get_original_race(d, c),
        card::get_original_attribute(d, c),
    )
}
/// The token data of `Duel.IsPlayerCanSpecialSummonMonster(tp,id+1,SET_CRYSTAL_BEAST,TYPES_TOKEN,s.getprops(c))`
fn token_md(d: &mut Duel, c: CardId) -> duel::MonsterData {
    let (atk, def, lv, race, attr) = getprops(d, c);
    duel::MonsterData {
        setcodes: Some(vec![SET_CRYSTAL_BEAST as u16]),
        type_: Some(TYPES_TOKEN),
        attack: Some(atk),
        defense: Some(def),
        level: Some(lv),
        race: Some(race),
        attribute: Some(attr),
    }
}
pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Destroy 1 "Crystal Beast" Monster Card
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY+CATEGORY_SPECIAL_SUMMON+CATEGORY_TOKEN)
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_SPECIAL_SUMMON + CATEGORY_TOKEN);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.destg)
    effect::set_target(d, e1, Some(cost_target(destg)));
    // e1:SetOperation(s.desop)
    effect::set_operation(d, e1, Some(operation(desop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon 1 "Crystal Beast" monster
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY + EFFECT_FLAG_DAMAGE_STEP, 0);
    // e2:SetCode(EVENT_MOVE)
    effect::set_code(d, e2, EVENT_MOVE);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(s.spcon)
    effect::set_condition(d, e2, Some(condition(spcon)));
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.desfilter(c,e,tp)`
fn desfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_CRYSTAL_BEAST) and c:IsOriginalType(TYPE_MONSTER) and Duel.GetMZoneCount(tp,c)>0
    //     and Duel.IsPlayerCanSpecialSummonMonster(tp,id+1,SET_CRYSTAL_BEAST,TYPES_TOKEN,s.getprops(c))
    if !(card::is_faceup(d, c)
        && card::is_set_card(d, c, &[SET_CRYSTAL_BEAST as u16])
        && card::is_original_type(d, c, TYPE_MONSTER))
    {
        return false;
    }
    if duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 <= 0 {
        return false;
    }
    let md = token_md(d, c);
    duel::is_player_can_special_summon_monster(d, tp, ID + 1, md, None, None, 0)
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_ONFIELD) and s.desfilter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_ONFIELD) && desfilter(d, chkc, tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.desfilter,tp,LOCATION_ONFIELD,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| desfilter(d, c, tp)), tp, LOCATION_ONFIELD, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,s.desfilter,tp,LOCATION_ONFIELD,0,1,1,nil,e,tp)
    let g = duel::select_target(d, tp, Some(&|d, c| desfilter(d, c, tp)), tp, LOCATION_ONFIELD, 0, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_TOKEN,nil,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, 1, 0, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and Duel.Destroy(tc,REASON_EFFECT)>0 and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsPlayerCanSpecialSummonMonster(tp,id+1,SET_CRYSTAL_BEAST,TYPES_TOKEN,s.getprops(tc)) then
    if card::is_relate_to_effect(d, tc, a.e) && duel::destroy(d, tc, REASON_EFFECT, None, None) > 0 && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0 {
        let md = token_md(d, tc);
        if duel::is_player_can_special_summon_monster(d, tp, ID + 1, md, None, None, 0) {
            // local token=Duel.CreateToken(tp,id+1)
            let token = duel::create_token(d, tp, ID + 1).nil();
            // Change Type, Attribute, Level, and ATK/DEF
            // token:Race(tc:GetOriginalRace())
            let race = card::get_original_race(d, tc);
            card::info_race(d, token, Some(race));
            // token:Attribute(tc:GetOriginalAttribute())
            let attr = card::get_original_attribute(d, tc);
            card::info_attribute(d, token, Some(attr));
            // token:Level(tc:GetOriginalLevel())
            let lv = card::get_original_level(d, tc);
            card::info_level(d, token, Some(lv));
            // token:Attack(math.max(0,tc:GetTextAttack()))
            let atk = card::get_text_attack(d, tc).max(0);
            card::info_attack(d, token, Some(atk));
            // token:Defense(math.max(0,tc:GetTextDefense()))
            let def = card::get_text_defense(d, tc).max(0);
            card::info_defense(d, token, Some(def));
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.SpecialSummon(token,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, token, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
/// `s.spconfilter(c,tp)`
fn spconfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_CRYSTAL_BEAST) and c:IsLocation(LOCATION_SZONE) and not c:IsPreviousLocation(LOCATION_SZONE)
    //     and c:IsControler(tp) and c:GetSequence()<5
    card::is_faceup(d, c)
        && card::is_set_card(d, c, &[SET_CRYSTAL_BEAST as u16])
        && card::is_location(d, c, LOCATION_SZONE)
        && !card::is_previous_location(d, c, LOCATION_SZONE)
        && card::is_controler(d, c, tp)
        && card::get_sequence(d, c) < 5
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg and eg:IsExists(s.spconfilter,1,nil,tp)
    let tp = a.tp;
    match a.eg.clone() {
        Some(eg) => group::is_exists(d, &eg, &|d, c| spconfilter(d, c, tp), 1, NO_EX),
        None => false,
    }
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_CRYSTAL_BEAST) and c:IsCanBeSpecialSummoned(e,0,tp,false,false) and c:IsOriginalType(TYPE_MONSTER)
    card::is_faceup(d, c)
        && card::is_set_card(d, c, &[SET_CRYSTAL_BEAST as u16])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
        && card::is_original_type(d, c, TYPE_MONSTER)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_SZONE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_SZONE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_SZONE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_SZONE as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<1 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 < 1 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_SZONE,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_SZONE, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, &g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
