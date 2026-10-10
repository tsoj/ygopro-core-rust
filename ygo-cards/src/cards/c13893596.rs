//! Exodius the Ultimate Forbidden Lord
use super::prelude::*;

pub const ID: u32 = 13893596;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_FORBIDDEN_ONE],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Special Summoning condition
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon procedure
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e2:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e2, EFFECT_SPSUMMON_PROC);
    // e2:SetRange(LOCATION_HAND)
    effect::set_range(d, e2, LOCATION_HAND);
    // e2:SetCondition(s.spcon)
    effect::set_condition(d, e2, Some(lua_fn(spcon)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(lua_fn(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Send 1 monster from your hand or Deck to the GY
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_TOGRAVE)
    effect::set_category(d, e3, CATEGORY_TOGRAVE);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e3:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e3, EVENT_ATTACK_ANNOUNCE);
    // e3:SetTarget(s.tgtg)
    effect::set_target(d, e3, Some(cost_target(tgtg)));
    // e3:SetOperation(s.tgop)
    effect::set_operation(d, e3, Some(operation(tgop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Gains 1000 ATK for each Normal Monster in your GY
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE);
    // e4:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e4, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e4:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e4, EFFECT_UPDATE_ATTACK);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetValue(function(e,c) return Duel.GetMatchingGroupCount(Card.IsType,c:GetControler(),LOCATION_GRAVE,0,nil,TYPE_NORMAL)*1000 end)
    effect::set_value_fn(d, e4, effect_card_value(atk_value));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // Banish itself it it leaves the field
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e5, EFFECT_TYPE_SINGLE);
    // e5:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e5, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e5:SetCode(EFFECT_LEAVE_FIELD_REDIRECT)
    effect::set_code(d, e5, EFFECT_LEAVE_FIELD_REDIRECT);
    // e5:SetCondition(function(e) return e:GetHandler():IsFaceup() end)
    effect::set_condition(d, e5, Some(lua_fn(redirect_cond)));
    // e5:SetValue(LOCATION_REMOVED)
    effect::set_value(d, e5, LOCATION_REMOVED as i64);
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
/// `function(e,c) return Duel.GetMatchingGroupCount(Card.IsType,c:GetControler(),LOCATION_GRAVE,0,nil,TYPE_NORMAL)*1000 end`
fn atk_value(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    let p = card::get_controler(d, c);
    duel::get_matching_group_count(d, Some(&|d, cc| card::is_type(d, cc, TYPE_NORMAL)), p, LOCATION_GRAVE, 0, NO_EX) as i64 * 1000
}
/// `function(e) return e:GetHandler():IsFaceup() end`
fn redirect_cond(d: &mut Duel, a: Args) -> bool {
    let h = effect::get_handler(d, a.e(0)).nil();
    card::is_faceup(d, h)
}
/// `s.spcon(e,c)`
fn spcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // local g=Duel.GetMatchingGroup(Card.IsMonster,tp,LOCATION_GRAVE,0,nil)
    let g = duel::get_matching_group(d, Some(&|d, cc| card::is_monster(d, cc)), tp, LOCATION_GRAVE, 0, NO_EX);
    // return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and #g>0 and #g==g:FilterCount(Card.IsAbleToDeckOrExtraAsCost,nil)
    duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && !g.is_empty()
        && g.len() == group::filter_count(d, &g, &|d, cc| card::is_able_to_deck_or_extra_as_cost(d, cc), NO_EX)
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp,c)`
fn spop(d: &mut Duel, a: Args) {
    let tp = a.p(1);
    // local g=Duel.GetMatchingGroup(Card.IsMonster,tp,LOCATION_GRAVE,0,nil)
    let g = duel::get_matching_group(d, Some(&|d, cc| card::is_monster(d, cc)), tp, LOCATION_GRAVE, 0, NO_EX);
    // Duel.SendtoDeck(g,nil,SEQ_DECKSHUFFLE,REASON_COST)
    duel::send_to_deck(d, g, None, SEQ_DECKSHUFFLE as i32, REASON_COST, None);
}
/// `s.tgtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_HAND|LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, (LOCATION_HAND | LOCATION_DECK) as i32);
    true
}
/// `s.winfilter(c,rc)`
fn winfilter(d: &mut Duel, c: CardId, rc: CardId) -> bool {
    // return c:IsRelateToCard(rc) and c:IsSetCard(SET_FORBIDDEN_ONE) and c:IsMonster()
    card::is_relate_to_card(d, c, rc) && card::is_set_card(d, c, &[SET_FORBIDDEN_ONE as u16]) && card::is_monster(d, c)
}
/// `s.tgop(e,tp,eg,ep,ev,re,r,rp)`
fn tgop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local tc=Duel.SelectMatchingCard(tp,aux.AND(Card.IsMonster,Card.IsAbleToGrave),tp,LOCATION_HAND|LOCATION_DECK,0,1,1,nil):GetFirst()
    let g = duel::select_matching_card(
        d,
        tp,
        Some(&|d, cc| card::is_monster(d, cc) && card::is_able_to_grave(d, cc, None)),
        tp,
        LOCATION_HAND | LOCATION_DECK,
        0,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    let tc = group::get_first(d, &g);
    // if tc and Duel.SendtoGrave(tc,REASON_EFFECT)>0 and tc:IsLocation(LOCATION_GRAVE) and c:IsRelateToEffect(e) then
    if let Some(tc) = tc {
        if duel::send_to_grave(d, CardSet::from_card(tc), REASON_EFFECT, None, None) > 0
            && card::is_location(d, tc, LOCATION_GRAVE)
            && card::is_relate_to_effect(d, c, a.e)
        {
            // tc:CreateRelation(c,RESET_EVENT|RESETS_STANDARD&~RESET_TURN_SET)
            card::create_relation(d, tc, c, RESET_EVENT | (RESETS_STANDARD & !RESET_TURN_SET));
            // Win condition
            // if not (c:IsFaceup() and c:IsOriginalCode(id) and c:IsControler(tp)) then return end
            if !(card::is_faceup(d, c) && card::is_original_code(d, c, &[ID]) && card::is_controler(d, c, tp)) {
                return;
            }
            // local g=Duel.GetMatchingGroup(s.winfilter,tp,LOCATION_GRAVE,0,nil,c)
            let g = duel::get_matching_group(d, Some(&|d, cc| winfilter(d, cc, c)), tp, LOCATION_GRAVE, 0, NO_EX);
            // if s.winfilter(tc,c) and g:GetClassCount(Card.GetCode)==5 then
            if winfilter(d, tc, c) && group::get_class_count(d, &g, &|d, cc| card::get_code(d, cc)[0] as i64) == 5 {
                // Duel.Win(tp,WIN_REASON_EXODIUS)
                duel::win(d, tp, WIN_REASON_EXODIUS);
            }
        }
    }
}
