//! V-LAN Hydra
//! V－LAN ヒドラ
use super::prelude::*;

pub const ID: u32 = 13536606;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[13536607], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // link summon
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link.AddProcedure(c,s.matfilter,2)
    link::add_procedure(d, c, Some(mat_filter(matfilter)), 2, None, None, None, None);
    // atk
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetValue(s.atkval)
    effect::set_value_fn(d, e1, effect_card_value(atkval));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // token
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOKEN)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOKEN);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetTarget(s.tktg)
    effect::set_target(d, e2, Some(cost_target(tktg)));
    // e2:SetOperation(s.tkop)
    effect::set_operation(d, e2, Some(operation(tkop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.matfilter(c,lc,sumtype,tp)`
fn matfilter(d: &mut Duel, c: CardId, lc: Option<CardId>, st: u64, tp: u8) -> bool {
    // return not c:IsType(TYPE_TOKEN,lc,sumtype,tp)
    !card::is_type_ctx(d, c, TYPE_TOKEN, lc, st, Some(tp))
}
/// `s.atkval(e,c)`: `#(c:GetMutualLinkedGroup():Filter(Card.IsMonster,nil))*300`
fn atkval(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    let g = card::get_mutual_linked_group(d, c);
    let m = group::filter(d, &g, &|d, x| card::is_monster(d, x), NO_EX);
    m.len() as i64 * 300
}
/// `s.rfilter(c,tp,g)`
fn rfilter(d: &mut Duel, c: CardId, tp: u8, g: &CardSet) -> bool {
    // local lk=c:GetLink()
    let lk = card::get_link(d, c) as i32;
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    // if c:IsControler(tp) then
    if card::is_controler(d, c, tp) {
        // return c:IsFaceup() and c:IsLinkMonster() and c:IsLinkBelow(3) and c:IsReleasableByEffect() and g:IsContains(c)
        //     and ((c:GetSequence()>4 and ft>=lk) or (c:GetSequence()<=4 and (ft+1)>=lk)) and (ft==1 or not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT))
        return card::is_faceup(d, c)
            && card::is_link_monster(d, c)
            && card::is_link_below(d, c, 3)
            && card::is_releasable_by_effect(d, c, None, None)
            && group::is_contains(g, c)
            && {
                let seq = card::get_sequence(d, c);
                (seq > 4 && ft >= lk) || (seq <= 4 && ft + 1 >= lk)
            }
            && (ft == 1 || duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_none());
    }
    // elseif c:IsControler(1-tp) then
    if card::is_controler(d, c, 1 - tp) {
        // return c:IsFaceup() and c:IsLinkMonster() and c:IsLinkBelow(3) and c:IsReleasableByEffect() and g:IsContains(c)
        //     and ft>=lk and (ft==1 or not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT))
        return card::is_faceup(d, c)
            && card::is_link_monster(d, c)
            && card::is_link_below(d, c, 3)
            && card::is_releasable_by_effect(d, c, None, None)
            && group::is_contains(g, c)
            && ft >= lk
            && (ft == 1 || duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_none());
    }
    // else return false
    false
}
/// `s.tktg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn tktg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local lg=c:GetMutualLinkedGroup()
    let lg = card::get_mutual_linked_group(d, c);
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and s.rfilter(chkc,tp,lg) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && rfilter(d, chkc, a.tp, &lg);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.rfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil,tp,lg)
    //     and Duel.IsPlayerCanSpecialSummonMonster(tp,id+1,0,TYPES_TOKEN,0,0,1,RACE_CYBERSE,ATTRIBUTE_LIGHT) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, x| rfilter(d, x, a.tp, &lg)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX)
            && duel::is_player_can_special_summon_monster(d, a.tp, ID + 1, token_data(), None, None, 0);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RELEASE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_RELEASE as u64);
    // local rg=Duel.SelectTarget(tp,s.rfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil,tp,lg)
    let rg = duel::select_target(d, a.tp, Some(&|d, x| rfilter(d, x, a.tp, &lg)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    // local ct=rg:GetFirst():GetLink()
    let first = group::get_first(d, &rg).nil();
    let ct = card::get_link(d, first);
    // Duel.SetOperationInfo(0,CATEGORY_TOKEN,nil,ct,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, ct as u8, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,ct,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, ct as u8, a.tp, 0);
    true
}
/// `s.tkop(e,tp,eg,ep,ev,re,r,rp)`
fn tkop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // local ct=tc:GetLink()
    let ct = card::get_link(d, tc.nil()) as i32;
    // if tc and tc:IsRelateToEffect(e) and not tc:IsImmuneToEffect(e) and Duel.Release(tc,REASON_EFFECT)>0 then
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e) && !card::is_immune_to_effect(d, tc, a.e) && duel::release(d, tc, REASON_EFFECT, None) > 0 {
            // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
            let ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
            // if ft<ct or (ft>1 and Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT)) then return end
            if ft < ct || (ft > 1 && duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some()) {
                return;
            }
            // if not Duel.IsPlayerCanSpecialSummonMonster(tp,id+1,0,TYPES_TOKEN,0,0,1,RACE_CYBERSE,ATTRIBUTE_LIGHT) then return end
            if !duel::is_player_can_special_summon_monster(d, a.tp, ID + 1, token_data(), None, None, 0) {
                return;
            }
            // for i=1,ct do
            for _ in 1..=ct {
                // local token=Duel.CreateToken(tp,id+1)
                let token = duel::create_token(d, a.tp, ID + 1).nil();
                // Duel.SpecialSummonStep(token,0,tp,tp,false,false,POS_FACEUP)
                duel::special_summon_step(d, token, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
            }
            // Duel.SpecialSummonComplete()
            duel::special_summon_complete(d);
        }
    }
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetLabel(ct)
    effect::set_label(d, e1, &[ct as i64]);
    // e1:SetTarget(s.splimit)
    effect::set_target(d, e1, Some(lua_fn(splimit)));
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
}
/// `s.splimit(e,c,sump,sumtype,sumpos,targetp,se)`
fn splimit(d: &mut Duel, a: Args) -> bool {
    // return c:IsLink(e:GetLabel())
    let lbl = effect::get_label(d, a.e(0));
    let c = a.c(1);
    card::is_link(d, c, &[lbl as u32])
}
/// The token of `Duel.IsPlayerCanSpecialSummonMonster(tp,id+1,0,TYPES_TOKEN,0,0,1,RACE_CYBERSE,ATTRIBUTE_LIGHT)`
fn token_data() -> duel::MonsterData {
    duel::MonsterData {
        setcodes: Some(vec![0]),
        type_: Some(TYPES_TOKEN),
        attack: Some(0),
        defense: Some(0),
        level: Some(1),
        race: Some(RACE_CYBERSE),
        attribute: Some(ATTRIBUTE_LIGHT),
    }
}
